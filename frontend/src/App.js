import { Game } from './components/Game/Game';
import { config } from "./config";
import './App.css';

function App() {
  return (
    <div>
      <Game {...config.gameConfig}/>
    </div>
  );
}

export default App;
