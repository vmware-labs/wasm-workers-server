// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

/**
 * A tic tac toe game!
 */
const html = (game, roomId) => `
<!DOCTYPE html>
<html>
  <head>
    <title>Tic Tac Toe!</title>
    <meta name="viewport" content="width=device-width,initial-scale=1">
    <meta charset="UTF-8">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/water.css@2/out/water.css">
    <style>
      .block {
        max-width: 600px;
        margin: 3rem auto 1rem;
        text-align: center;
      }

      .game {
        display: grid;
        width: 280px;
        margin: 1rem auto;
        grid-template-columns: repeat(3, 1fr);
        grid-template-rows: repeat(3, 1fr);
        gap: 1rem;
      }

      .game-tile {
        height: 60px;
      }

      .game-tile button {
        height: 60px;
        width: 100%;
        margin: 0;
      }

      .alert {
        margin: 1rem auto;
        color: white;
        padding: .5rem;
        display: none;
      }

      .waiting {
        background-color: #866320;
      }

      .win {
        background-color: #1d6a1d;
      }

      .lose {
        background-color: #682525;
      }

      .turn {
        background-color: #264c95;
      }

      .room {
        display: none;
        width: 800px;
        margin: 0 auto;
        text-align: center;
      }

      .alert.show,
      .room.show {
        display: block;
      }
    </style>
  </head>
  <body>
    <main>
      <div class="block">
        <h1>Welcome to Tic Tac Toe ❌ ⭕️</h1>
        <p>This game is a demo of Wasm Web Workers! Hope you enjoy it</p>
        <div class="alert waiting">Waiting next movement</div>
        <div class="alert turn">Your turn</div>
        <div class="alert win">You win :)!</div>
        <div class="alert lose">You lose :(!</div>
        <div class="game"></div>
      </div>
      <pre class="room"><code class="url"></code></pre>
    </main>
    <script>
      const game = document.querySelector(".game");
      let moves = JSON.parse('${JSON.stringify(game.movements)}');
      let turn = moves.filter(move => move != "").length % 2 == 0 ? "x" : "o";
      let waiting = ${game.waiting};
      let finished = false;
      let symbol = !waiting ? turn : (turn == "x" ? "o" : "x");
      let pullInterval;

      const getUrl = () => {
        let url = window.location.href;
        if (!url.includes(${roomId})) {
          url = url + "?room=${roomId}";
        }

        return url;
      }

      const updateMovements = (moves) => {
        moves.forEach((move, i) => {
          let button = document.querySelector(\`button[data-cell="\${i}"]\`);

          if (move == "x") {
            button.innerText = "❌";
          } else if (move == "o") {
            button.innerText = "⭕️";
          }
        });
      }

      const sameSymbol = (arr) => arr.every(s => s == arr[0] && s != "");

      const winnerSymbol = (moves) => {
        let winner;

        // Horizontal
        for (let i = 0; i < moves.length; i += 3) {
          const chunk = moves.slice(i, i + 3);
          if (sameSymbol(chunk)) {
            winner = chunk[0];
            break;
          }
        }

        if (winner) return winner;

        // Vertical
        for (let i = 0; i < 3; i++) {
          const chunk = [moves[i], moves[i+3], moves[i+6]];
          if (sameSymbol(chunk)) {
            winner = chunk[0];
            break;
          }
        }

        if (winner) return winner;

        // Cross!
        if (sameSymbol([moves[0], moves[4], moves[8]]) ||
            sameSymbol([moves[2], moves[4], moves[6]])) {
          winner = moves[4];
        }

        return winner;
      }

      const checkFinish = (moves) => {
        const winner = winnerSymbol(moves);
        finished = winner != null;

        if (winner == symbol) {
          document.querySelector(".win").classList.toggle("show");
        } else if (winner != null) {
          document.querySelector(".lose").classList.toggle("show");
        }

        if (finished) {
          document.querySelector(".waiting").classList.remove("show");
          document.querySelector(".turn").classList.remove("show");
        }

        return finished;
      }

      const setWaiting = () => {
        document.querySelector(".waiting").classList.toggle("show");
        document.querySelector(".turn").classList.remove("show");
        pullInterval = setInterval(pullData, 1000);
      }

      const pullData = () => {
        fetch(
          getUrl(),
          {
            headers: {
              "Accept": "application/json"
            }
          }
        ).then(res => res.json())
         .then(json => {
           if (json.toString() != moves.toString()) {
             clearInterval(pullInterval);
             moves = json;
             updateMovements(moves);
             waiting = false;
             document.querySelector(".turn").classList.toggle("show");
             document.querySelector(".waiting").classList.toggle("show");

             // Check for winners now!
             checkFinish(moves);
           }
         });
      }

      if (waiting) {
        setWaiting();
      } else {
        document.querySelector(".turn").classList.add("show");
        document.querySelector(".url").innerText = "Share to play: " + getUrl();
        document.querySelector(".room").classList.add("show");
      }

      // Init the board
      moves.forEach((move, i) => {
        const node = document.createElement("div");
        node.classList.add("game-tile");
        const button = document.createElement("button");
        button.dataset.cell = i;

        if (move == "x") {
          button.innerText = "❌";
        } else if (move == "o") {
          button.innerText = "⭕️";
        }

        node.append(button);
        game.append(node);
      });

      document.querySelectorAll(".game-tile button").forEach(button => {
        button.addEventListener("click", (event) => {
          if (waiting || finished || event.target.innerText != "") {
            return;
          }

          if (symbol == "x") {
            event.target.innerText = "❌";
          } else {
            event.target.innerText = "⭕️";
          }

          let cell = button.dataset.cell;

          if (!waiting) {
            sendMove(cell, symbol);
          }
        });
      });

      const sendMove = (cell, symbol) => {
        moves[cell] = symbol;

        fetch(
          getUrl(),
          {
            method: "PUT",
            headers: {
              "Content-Type": "application/json"
            },
            body: JSON.stringify(moves)
          }
        ).then(res => res.json())
         .then(json => {
           let finished = checkFinish(moves);

           if (!finished) {
             setWaiting();
           }
         });
      }
    </script>
  </body>
</html.
`;

const setCache = (key, data) => Cache.set(key, data);
const getCache = key => Cache.get(key);

const getRoomId = (request) => {
  // Get Room id
  let roomId = request.url.split("?room=")[1];

  if (!roomId) {
    roomId = (Math.random() + 1).toString(36).substring(7);
  }

  return roomId;
}

const getGame = (roomId) => {
  // Get Room id
  let movements = getCache(roomId);
  let waiting = false;

  if (!movements) {
    movements = new Array(9).fill("");
    // Save in the cache!
    setCache(roomId, JSON.stringify(movements));
  } else {
    movements = JSON.parse(movements);
    waiting = true;
  }

  return {
    waiting,
    movements
  };
}

/**
 * Builds a reply to the given request
 */
const site = (request) => {
  // Get Room id and moves
  let roomId = getRoomId(request);
  let game = getGame(roomId);

  // Build a new response
  let response = new Response(html(game, roomId));

  // Add a new header
  response.headers.set("x-generated-by", "wasm-workers-server");

  return response;
}

const moveStatus = (request) => {
  let roomId = getRoomId(request);
  let { movements } = getGame(roomId);

  let response = new Response(JSON.stringify(movements), {
    headers: {
      "content-type": "application/json;charset=UTF-8",
      "x-generated-by": "wasm-workers-server"
    }
  });

  return response;
};

const update = (request) => {
  let roomId = getRoomId(request);
  let movements = request.body;

  setCache(roomId, movements);

  let response = new Response(movements, {
    headers: {
      "Content-Type": "application/json",
      "x-generated-by": "wasm-workers-server"
    }
  });

  return response;
};

const route = (request) => {
  if (request.method == "GET") {
    if (request.headers.get("accept") == "application/json") {
      return moveStatus(request);
    } else {
      return site(request);
    }
  } else if (request.method == "PUT") {
    return update(request);
  }

  return new Response("Method not allowed", {
    status: 405
  });
};

// Subscribe to the Fetch event
addEventListener("fetch", event => {
  return event.respondWith(route(event.request));
});
