import css from "./Game.module.css";
import { Tile } from "./../Tile/Tile";

export function Game(props) {
    const boardStyle = {
        gridTemplateColumns: `repeat(${props.columns}, 1fr)`,
        gridTemplateRows: `repeat(${props.rows}, 1fr)`,
    }

    const tiles = [];
    for (let rowId = 0; rowId < props.rows; rowId += 1) {
        for (let columnId = 0; columnId < props.columns; columnId += 1) {
            tiles.push(<Tile key={rowId * props.columns + columnId}/>);
        }
    }

    return (
        <div className={css.field}>
            <div className={css.header}>
                <div className={css.headerTurn}></div>
                <div className={css.headerStatus}></div>
                <button type="button" className={css.headerRestart}>
                    <i className="material-icons">refresh</i>
                </button>
            </div>
            <div className={css.board} style={boardStyle}>
                {tiles}
            </div>
        </div>
    );
}