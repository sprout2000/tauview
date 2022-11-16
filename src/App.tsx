import {
  Fragment,
  useCallback,
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
} from 'react';

import { event } from '@tauri-apps/api';
import { dirname } from '@tauri-apps/api/path';
import { getCurrent } from '@tauri-apps/api/window';
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';

import L from 'leaflet';
import 'leaflet/dist/leaflet.css';

import { ToolBar } from './ToolBar';
import './App.scss';

type Payload = {
  message: string | null;
};

export const App = () => {
  const [url, setUrl] = useState('');
  const [grid, setGrid] = useState(false);
  const [imgList, setImgList] = useState<string[]>([]);

  const mapRef = useRef<HTMLDivElement>(null);
  const currentRef = useRef<HTMLImageElement>(null);
  const mapObj: React.MutableRefObject<L.Map | null> = useRef(null);

  const isDev = process.env.NODE_ENV === 'development';

  const readDir = useCallback(async () => {
    const dir = await dirname(url);
    const list: string[] = await invoke('get_entries', { dir });

    return list;
  }, [url]);

  const getZoom = useCallback(
    (iw: number, width: number, ih: number, height: number) => {
      if (iw > width || ih > height) {
        const zoomX = width / iw;
        const zoomY = height / ih;

        return zoomX >= zoomY ? zoomY : zoomX;
      } else {
        return 1;
      }
    },
    []
  );

  const draw = useCallback(
    (width: number, height: number) => {
      const node = mapRef.current;

      if (node) {
        const image = new Image();
        image.onload = () => {
          const zoom = getZoom(image.width, width, image.height, height);

          const bounds = new L.LatLngBounds([
            [image.height * zoom, 0],
            [0, image.width * zoom],
          ]);

          mapObj.current?.off();
          mapObj.current?.remove();

          mapObj.current = L.map(node, {
            maxBounds: bounds,
            crs: L.CRS.Simple,
            preferCanvas: true,
            zoomSnap: 0.3,
            zoomDelta: 0.3,
            wheelPxPerZoomLevel: 360,
            doubleClickZoom: false,
            zoomControl: false,
            attributionControl: false,
          }).fitBounds(bounds);

          mapObj.current.on('dblclick', () => {
            mapObj.current?.setView(bounds.getCenter(), 0);
          });

          if (image.width < width && image.height < height) {
            mapObj.current.setView(bounds.getCenter(), 0, { animate: false });
          }

          L.imageOverlay(image.src, bounds).addTo(mapObj.current);

          node.blur();
          node.focus();
        };
        image.src = url ? convertFileSrc(url) : '';
      }
    },
    [url, getZoom]
  );

  const preventDefault = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
  };

  const onOpen = useCallback(() => {
    invoke('open_dialog')
      .then((fpath) => {
        if (typeof fpath === 'string') {
          setUrl(fpath);
          setGrid(false);
        }
      })
      .catch((err) => console.log(`Error: ${err}`));
  }, []);

  const onNext = useCallback(async () => {
    if (!url) return;

    const list = await readDir();
    if (!list || list.length === 0) {
      window.location.reload();
      return;
    }
    if (list.length === 1) return;

    const index = list.indexOf(url);
    if (index === list.length - 1 || index === -1) {
      setUrl(list[0]);
    } else {
      setUrl(list[index + 1]);
    }
  }, [url, readDir]);

  const onPrev = useCallback(async () => {
    if (!url) return;

    const list = await readDir();
    if (!list || list.length === 0) {
      window.location.reload();
      return;
    }
    if (list.length === 1) return;

    const index = list.indexOf(url);
    if (index === 0) {
      setUrl(list[list.length - 1]);
    } else if (index === -1) {
      setUrl(list[0]);
    } else {
      setUrl(list[index - 1]);
    }
  }, [url, readDir]);

  const onRemove = useCallback(async () => {
    if (!url) return;

    const list = await readDir();
    if (!list || list.length === 0) {
      window.location.reload();
      return;
    }

    const index = list.indexOf(url);
    await invoke('move_to_trash', { url }).catch((err) => {
      console.log(`Error in move_to_trash(): ${err}`);
      window.location.reload();
      return;
    });

    const newList = await readDir();
    if (!newList || newList.length === 0) {
      window.location.reload();
      return;
    }

    setImgList(newList);

    if (index > newList.length - 1) {
      setUrl(newList[0]);
    } else {
      setUrl(newList[index]);
    }
  }, [url, readDir]);

  const onToggleGrid = useCallback(async () => {
    if (!url) return;

    const list = await readDir();
    if (!list || list.length === 0 || !list.includes(url)) {
      window.location.reload();
      return;
    }

    if (!grid) {
      setImgList(list);
      setGrid(true);
    } else {
      setGrid(false);
    }
  }, [grid, url, readDir]);

  const onClickThumb = async (
    e: React.MouseEvent<HTMLImageElement, MouseEvent>,
    item: string
  ) => {
    e.stopPropagation();

    const list = await readDir();
    if (!list || list.length === 0 || !list.includes(item)) {
      window.location.reload();
      return;
    }

    setUrl(item);
    setGrid(false);
  };

  const onClickBlank = async () => {
    const list = await readDir();
    if (!list || list.length === 0 || !list.includes(url)) {
      window.location.reload();
      return;
    }

    setGrid(false);
  };

  const onKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (!url || grid) return;

    switch (e.key) {
      case '0':
        e.preventDefault();
        mapObj.current?.setZoom(0);
        break;
      case e.metaKey && 'ArrowRight':
      case e.ctrlKey && 'n':
        e.preventDefault();
        onNext();
        break;
      case e.metaKey && 'ArrowLeft':
      case e.ctrlKey && 'p':
        e.preventDefault();
        onPrev();
        break;
    }
  };

  useEffect(() => {
    const unlisten = event.listen('menu-next', () => {
      if (grid) setGrid(false);
      onNext();
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, [onNext, grid]);

  useEffect(() => {
    const unlisten = event.listen('menu-prev', () => {
      if (grid) setGrid(false);
      onPrev();
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, [onPrev, grid]);

  useEffect(() => {
    const unlisten = event.listen('menu-grid', () => {
      onToggleGrid();
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, [onToggleGrid]);

  useEffect(() => {
    const unlisten = event.listen('menu-remove', () => {
      onRemove();
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, [onRemove]);

  useEffect(() => {
    const unlisten = event.listen(
      'tauri://file-drop',
      async (e: event.Event<string[]>) => {
        if (grid) {
          return false;
        }

        const filepath = e.payload[0];
        const mimeSafe: boolean = await invoke('mime_check', { filepath });
        if (!mimeSafe) return;

        setUrl(filepath);
      }
    );

    return () => {
      unlisten.then((f) => f());
    };
  }, [grid]);

  useEffect(() => {
    const unlisten = event.listen('open', (e: event.Event<Payload>) => {
      const filepath = e.payload.message;
      if (!filepath) return;
      if (grid) setGrid(false);

      setUrl(filepath);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, [grid]);

  useEffect(() => {
    const currentWindow = getCurrent();

    if (!url) {
      currentWindow.setTitle('LeafView');
    } else {
      currentWindow.setTitle(url.replace(/.+(\/|\\)/, ''));
    }
  }, [url]);

  useEffect(() => {
    const resizeObserver = new ResizeObserver((entries) => {
      const width = entries[0].contentRect.width;
      const height = entries[0].contentRect.height;
      draw(width, height);
    });

    mapRef.current && resizeObserver.observe(mapRef.current);

    return () => {
      resizeObserver.disconnect();
    };
  }, [draw, grid]);

  useLayoutEffect(() => {
    if (grid)
      currentRef.current?.scrollIntoView({
        behavior: 'smooth',
        block: 'center',
      });
  }, [grid]);

  return (
    <div
      data-testid="container"
      className={grid ? 'container grid' : 'container'}
      onKeyDown={onKeyDown}
      onDrop={preventDefault}
      onDragOver={preventDefault}
      onDragEnter={preventDefault}
      onDragLeave={preventDefault}
      onContextMenu={isDev ? undefined : preventDefault}
    >
      {grid ? (
        <div
          className="thumb-container"
          onClick={onClickBlank}
          onKeyDown={(e) => {
            e.preventDefault();
          }}
        >
          {imgList.map((item) => (
            <img
              key={item}
              src={convertFileSrc(item)}
              ref={item === url ? currentRef : null}
              className={item === url ? 'thumb current' : 'thumb'}
              onClick={(e) => onClickThumb(e, item)}
              onDragStart={() => {
                return false;
              }}
            />
          ))}
        </div>
      ) : (
        <Fragment>
          <ToolBar
            onOpen={onOpen}
            onPrev={onPrev}
            onNext={onNext}
            onRemove={onRemove}
            onToggleGrid={onToggleGrid}
          />
          <div className={url ? 'view' : 'view init'} ref={mapRef} />
        </Fragment>
      )}
    </div>
  );
};
