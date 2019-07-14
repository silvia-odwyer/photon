import React from 'react';
import './App.css';

import Canvas from "./components/Canvas"

const App = () => {

  return (
    <div className="App">
      <header className="App-header">
      <nav class="topnav">
        <ul>
          <li><a href="https://silvia-odwyer.github.io/photon/docs/photon/index.html">Docs</a></li>
          <li><a href="https://github.com/silvia-odwyer/photon">GitHub</a></li>
        </ul>
            
        </nav>

        <Canvas></Canvas>
      </header>

    </div>
  );
};

export default App;