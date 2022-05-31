import css from "./Tile.module.css";

export function Tile(props) {
    return (
        <div className={css.tile}>{props.value}</div>
    )
}