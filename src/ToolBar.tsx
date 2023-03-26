import { memo } from "react";
import * as i18next from "i18next";

import { Trash } from "./icons/Trash";
import { GridIcon } from "./icons/GridIcon";
import { ArrowLeft } from "./icons/ArrowLeft";
import { ArrowRight } from "./icons/ArrowRight";
import { FolderOpen } from "./icons/FolderOpen";

import "./ToolBar.scss";

type Props = {
  onOpen: () => void;
  onPrev: () => Promise<void>;
  onNext: () => Promise<void>;
  onRemove: () => Promise<void>;
  onToggleGrid: () => Promise<void>;
};

export const ToolBar = memo((props: Props) => {
  return (
    <div className="bottom">
      <div className="folder">
        <div
          aria-label="open-button"
          className="icon"
          title={`${i18next.t("Open...")}`}
          onClick={props.onOpen}
        >
          <FolderOpen />
        </div>
      </div>
      <div className="toolbar">
        <div className="grid">
          <div
            aria-label="grid-button"
            className="icon"
            title={`${i18next.t("Toggle Grid View")}`}
            onClick={props.onToggleGrid}
          >
            <GridIcon />
          </div>
        </div>
        <div className="arrows">
          <div
            aria-label="prev-button"
            className="icon"
            title={`${i18next.t("Prev Image")}`}
            onClick={props.onPrev}
          >
            <ArrowLeft />
          </div>
          <div
            aria-label="next-button"
            className="icon"
            title={`${i18next.t("Next Image")}`}
            onClick={props.onNext}
          >
            <ArrowRight />
          </div>
        </div>
        <div className="trash">
          <div
            aria-label="trash-button"
            className="icon"
            title={`${i18next.t("Move to Trash")}`}
            onClick={props.onRemove}
          >
            <Trash />
          </div>
        </div>
      </div>
    </div>
  );
});

ToolBar.displayName = "ToolBar";
