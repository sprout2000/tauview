import { convertFileSrc } from "@tauri-apps/api/core";
import { memo, useLayoutEffect, useRef } from "react";

import "./Grid.scss";

type Props = {
  url: string;
  imgList: string[];
  onClickBlank: () => Promise<void>;
  onClickThumb: (
    e: React.MouseEvent<HTMLButtonElement, MouseEvent>,
    item: string,
  ) => Promise<void>;
};

export const Grid = memo((props: Props) => {
  const currentRef = useRef<HTMLDivElement>(null);
  const currentButtonRef = useRef<HTMLButtonElement>(null);

  useLayoutEffect(() => {
    currentRef.current?.scrollIntoView({
      behavior: "smooth",
      // block: "center",
    });
    // Escape でグリッドを閉じられるように、表示中の画像のサムネイルにフォーカスする
    currentButtonRef.current?.focus({ preventScroll: true });
  }, []);

  const handleKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (e.key === "Escape") props.onClickBlank();
  };

  return (
    <div
      className="grid-content"
      role="presentation"
      onClick={props.onClickBlank}
      onKeyDown={handleKeyDown}
    >
      <div className="thumb-container">
        {props.imgList.map((item) => (
          <div
            className="wrapper"
            key={item}
            ref={item === props.url ? currentRef : null}
          >
            <button
              type="button"
              className="thumb-button"
              ref={item === props.url ? currentButtonRef : null}
              onClick={(e) => props.onClickThumb(e, item)}
            >
              <img
                alt={item.replace(/.+(\/|\\)/, "")}
                src={convertFileSrc(item, "tauview")}
                className={item === props.url ? "thumb current" : "thumb"}
                draggable={false}
              />
            </button>
          </div>
        ))}
      </div>
    </div>
  );
});

Grid.displayName = "Grid";
