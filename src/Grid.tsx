import { convertFileSrc } from "@tauri-apps/api/core";
import { memo, useEffect, useLayoutEffect, useRef } from "react";

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
    // キーボードで操作できるように、表示中の画像のサムネイルにフォーカスする
    currentButtonRef.current?.focus({ preventScroll: true });
  }, []);

  const contentRef = useRef<HTMLDivElement>(null);
  const { onClickBlank } = props;

  useEffect(() => {
    const node = contentRef.current;

    // サムネイル以外の余白をクリックしたらグリッドを閉じる
    const handleClick = (e: MouseEvent) => {
      if ((e.target as Element).closest(".thumb-button")) return;
      onClickBlank();
    };

    // Escape でグリッドを閉じる
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClickBlank();
    };

    node?.addEventListener("click", handleClick);
    window.addEventListener("keydown", handleKeyDown);
    return () => {
      node?.removeEventListener("click", handleClick);
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [onClickBlank]);

  return (
    <div className="grid-content" ref={contentRef}>
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
                src={`${convertFileSrc(item, "tauview")}?size=thumb`}
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
