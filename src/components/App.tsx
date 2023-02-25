import { useCallback, useEffect, useState } from 'react';

import { event } from '@tauri-apps/api';
import { invoke } from '@tauri-apps/api/tauri';
import { dirname } from '@tauri-apps/api/path';
import { getCurrent } from '@tauri-apps/api/window';

import { View } from './View';
import { Grid } from './Grid';
import { ToolBar } from './ToolBar';

import './App.scss';

type Payload = {
  message: string | null;
};

export const App = () => {
  const [url, setUrl] = useState('');
  const [grid, setGrid] = useState(false);
  const [imgList, setImgList] = useState<string[]>([]);

  const readDir = useCallback(async () => {
    const dir = await dirname(url);
    const list: string[] = await invoke('get_entries', { dir });

    return list;
  }, [url]);

  const preventDefault = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
  };

  const handleOpen = useCallback(() => {
    invoke('open_dialog')
      .then((fpath) => {
        if (typeof fpath === 'string') {
          setUrl(fpath);
          setGrid(false);
        }
      })
      .catch((err) => console.log(`Error: ${err}`));
  }, []);

  const handleNext = useCallback(async () => {
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

  const handlePrev = useCallback(async () => {
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

  const handleRemove = useCallback(async () => {
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

  const handleToggleGrid = useCallback(async () => {
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

  const handleClickThumb = async (
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

  const handleClickBlank = async () => {
    const list = await readDir();
    if (!list || list.length === 0 || !list.includes(url)) {
      window.location.reload();
      return;
    }

    setGrid(false);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (!url || grid) return;

    switch (e.key) {
      case e.metaKey && 'ArrowRight':
      case e.ctrlKey && 'n':
        e.preventDefault();
        handleNext();
        break;
      case e.metaKey && 'ArrowLeft':
      case e.ctrlKey && 'p':
        e.preventDefault();
        handlePrev();
        break;
    }
  };

  useEffect(() => {
    const unlisten = event.listen('menu-next', () => {
      if (grid) setGrid(false);
      handleNext();
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, [handleNext, grid]);

  useEffect(() => {
    const unlisten = event.listen('menu-prev', () => {
      if (grid) setGrid(false);
      handlePrev();
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, [handlePrev, grid]);

  useEffect(() => {
    const unlisten = event.listen('menu-grid', () => {
      handleToggleGrid();
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, [handleToggleGrid]);

  useEffect(() => {
    const unlisten = event.listen('menu-remove', () => {
      handleRemove();
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, [handleRemove]);

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
      currentWindow.setTitle('LeafTauView');
    } else {
      currentWindow.setTitle(url.replace(/.+(\/|\\)/, ''));
    }
  }, [url]);

  return (
    <div
      data-testid="container"
      className={grid ? 'container grid' : 'container'}
      onKeyDown={handleKeyDown}
      onDrop={preventDefault}
      onDragOver={preventDefault}
      onDragEnter={preventDefault}
      onDragLeave={preventDefault}
      onContextMenu={preventDefault}
    >
      {grid ? (
        <Grid
          url={url}
          imgList={imgList}
          onClickBlank={handleClickBlank}
          onClickThumb={handleClickThumb}
        />
      ) : (
        <>
          <ToolBar
            onOpen={handleOpen}
            onPrev={handlePrev}
            onNext={handleNext}
            onRemove={handleRemove}
            onToggleGrid={handleToggleGrid}
          />
          <View url={url} />
        </>
      )}
    </div>
  );
};
