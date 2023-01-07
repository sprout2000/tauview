import { memo } from 'react';
import * as i18next from 'i18next';

import { Grid } from './icons/Grid';
import { Trash } from './icons/Trash';
import { ArrowLeft } from './icons/ArrowLeft';
import { ArrowRight } from './icons/ArrowRight';
import { FolderOpen } from './icons/FolderOpen';

import './ToolBar.scss';

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
          data-testid="open-button"
          className="icon"
          title={i18next.t('Open...') as string}
          onClick={props.onOpen}
        >
          <FolderOpen />
        </div>
      </div>
      <div className="toolbar">
        <div className="grid">
          <div
            className="icon"
            title={i18next.t('Toggle Grid View') as string}
            onClick={props.onToggleGrid}
          >
            <Grid />
          </div>
        </div>
        <div className="arrows">
          <div
            data-testid="prev-button"
            className="icon"
            title={i18next.t('Prev Image') as string}
            onClick={props.onPrev}
          >
            <ArrowLeft />
          </div>
          <div
            data-testid="next-button"
            className="icon"
            title={i18next.t('Next Image') as string}
            onClick={props.onNext}
          >
            <ArrowRight />
          </div>
        </div>
        <div className="trash">
          <div
            data-testid="trash-button"
            className="icon"
            title={i18next.t('Move to Trash') as string}
            onClick={props.onRemove}
          >
            <Trash />
          </div>
        </div>
      </div>
    </div>
  );
});

ToolBar.displayName = 'ToolBar';
