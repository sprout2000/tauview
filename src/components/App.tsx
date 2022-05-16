import React, { useState, useRef, useCallback, useEffect } from "react";

import { event } from "@tauri-apps/api";
import { dirname } from "@tauri-apps/api/path";
import { getCurrent } from "@tauri-apps/api/window";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";

import L from "leaflet";
import "leaflet/dist/leaflet.css";

import { ToolBar } from "./ToolBar";
import "./App.scss";

type Payload = {
  message: string | null;
};

export const App = () => {
  const [url, setUrl] = useState("");

  const mapRef = useRef<HTMLDivElement>(null);
  const mapObj: React.MutableRefObject<L.Map | null> = useRef(null);

  const readDir = useCallback(async () => {
    const dir = await dirname(url);

    const list: string[] = await invoke("get_entries", {
      dir: dir.replace("asset://", ""),
    });

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
            zoomDelta: 0.3,
            zoomSnap: 0.3,
            wheelPxPerZoomLevel: 360,
            doubleClickZoom: false,
            zoomControl: false,
            attributionControl: false,
          }).fitBounds(bounds);

          mapObj.current.on("dblclick", () => {
            mapObj.current?.setView(bounds.getCenter(), 0);
          });

          if (image.width < width && image.height < height) {
            mapObj.current.setView(bounds.getCenter(), 0, { animate: false });
          }

          L.imageOverlay(image.src, bounds).addTo(mapObj.current);

          node.blur();
          node.focus();
        };
        image.src = url ? convertFileSrc(url) : "";
      }
    },
    [url, getZoom]
  );

  const preventDefault = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
  };

  const onOpen = useCallback(async () => {
    await invoke("open_dialog")
      .then((fpath) => {
        if (typeof fpath === "string") setUrl(fpath);
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
    await invoke("move_to_trash", { url }).catch((err) => {
      console.log(`Error in move_to_trash(): ${err}`);
      window.location.reload();
      return;
    });

    const newList = await readDir();
    if (!newList || newList.length === 0) {
      window.location.reload();
      return;
    }

    if (index > newList.length - 1) {
      setUrl(newList[0]);
    } else {
      setUrl(newList[index]);
    }
  }, [url, readDir]);

  const onKeyDown = (e: React.KeyboardEvent) => {
    if (!url) return;

    switch (e.key) {
      case "0":
        e.preventDefault();
        mapObj.current && mapObj.current.setZoom(0);
        break;
      case e.metaKey && "ArrowRight":
      case "j":
        e.preventDefault();
        onNext();
        break;
      case e.metaKey && "ArrowLeft":
      case "k":
        e.preventDefault();
        onPrev();
        break;
      case "Delete":
        e.preventDefault();
        onRemove();
        break;
      default:
        return;
    }
  };

  useEffect(() => {
    const unlisten = event.listen(
      "tauri://file-drop",
      async (e: event.Event<string[]>) => {
        const filepath = e.payload[0];
        const mimeSafe: boolean = await invoke("mime_check", { filepath });
        if (!mimeSafe) return;

        setUrl(filepath);
      }
    );

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  useEffect(() => {
    const unlisten = event.listen("open", (e: event.Event<Payload>) => {
      const filepath = e.payload.message;
      if (!filepath) return;

      setUrl(filepath);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  useEffect(() => {
    const currentWindow = getCurrent();

    if (!url) {
      currentWindow.setTitle("LeafView");
    } else {
      currentWindow.setTitle(url.replace(/.+(\/|\\)/, ""));
    }
  }, [url]);

  useEffect(() => {
    const resizeObserver = new ResizeObserver((entries) => {
      const width = entries[0].contentRect.width;
      const height = entries[0].contentRect.height;
      draw(width, height);
    });

    mapRef.current && resizeObserver.observe(mapRef.current);

    return () => resizeObserver.disconnect();
  }, [draw]);

  return (
    <div
      className="container"
      onKeyDown={onKeyDown}
      onDrop={preventDefault}
      onDragOver={preventDefault}
      onDragEnter={preventDefault}
      onDragLeave={preventDefault}
    >
      <div className="bottom">
        <ToolBar
          onOpen={onOpen}
          onPrev={onPrev}
          onNext={onNext}
          onRemove={onRemove}
        />
      </div>
      <div className={url ? "view" : "view init"} ref={mapRef} />
    </div>
  );
};
