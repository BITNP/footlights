import logo from './logo.svg';
import './App.css';
import init, { Engine, greet, new_structure, new_style } from 'footlights-wasm';
import { useEffect, useState } from 'react';

const withWasm = (fn) => {
  init().then(() => {
    fn();
  })
}

function App() {
  let [str, setStr] = useState(null);

  useEffect(() => {
    withWasm(() => {
      let engine = new Engine();

      let bg = new_style();
      let img = new_style();

      img.image = logo
      img.round = 20

      bg.color = {
        Linear: {
          stops: [
            // - - hsl(240 46% 65%)
            //   - 0%
            ["hsl(240 46% 65%)", "0%"],
            // - - hsl(259 43% 64%)
            //   - 10%
            ["hsl(259 43% 64%)", "10%"],
            // - - hsl(295 34% 63%)
            //   - 22%
            ["hsl(295 34% 63%)", "22%"],
            // - - hsl(313 39% 65%)
            //   - 27%
            ["hsl(313 39% 65%)", "27%"],
            // - - hsl(325 48% 68%)
            //   - 32%
            ["hsl(325 48% 68%)", "32%"],
            // - - hsl(335 55% 70%)
            //   - 36%
            ["hsl(335 55% 70%)", "36%"],
            // - - hsl(343 60% 73%) 
            //   - 41%
            ["hsl(343 60% 73%)", "41%"],
            // - - hsl(351 64% 75%) 
            //   - 45%
            ["hsl(351 64% 75%)", "45%"],
            // - - hsl(359 66% 77%) 
            //   - 49%
            ["hsl(359 66% 77%)", "49%"],
            // - - hsl(6 68% 77%) 
            //   - 53%
            ["hsl(6 68% 77%)", "53%"],
            // - - hsl(12 69% 77%) 
            //   - 58%
            ["hsl(12 69% 77%)", "58%"],
            // - - hsl(18 68% 78%) 
            //   - 62%
            ["hsl(18 68% 78%)", "62%"],
            // - - hsl(23 65% 78%) 
            //   - 67%
            ["hsl(23 65% 78%)", "67%"],
            // - - hsl(28 62% 78%) 
            //   - 72%
            ["hsl(28 62% 78%)", "72%"],
            // - - hsl(33 57% 79%) 
            //   - 77%
            ["hsl(33 57% 79%)", "77%"],
            // - - hsl(39 51% 81%) 
            //   - 83%
            ["hsl(39 51% 81%)", "83%"],
            // - - hsl(46 44% 82%) 
            //   - 91%
            ["hsl(46 44% 82%)", "91%"],
            // - - hsl(56 37% 84%) 
            //   - 100%
            ["hsl(56 37% 89%)", "100%"],

          ],
          degree: 35,
        }
      }

      console.log(bg)
      engine.add_style("bg", bg);
      engine.add_style("img", img);


      let str = engine.render();
      console.log(str);
      setStr(str);
    })
  },)



  return (
    <div className="App">
      <header className="App-header">
        <div dangerouslySetInnerHTML={{ __html: str }}></div>
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React

        </a>
      </header>
    </div>
  );
}

export default App;
