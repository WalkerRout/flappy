import * as sim from "lib-simulation-wasm";

const simulation = new sim.Simulation();
const viewport = document.getElementById("viewport");
const ticks = document.getElementById("ticks");
const generations = document.getElementById("generations");
const trainButton = document.getElementById("train");
let generation_count = 0;

// Absolute
/*const viewportWidth = viewport.width;
const viewportHeight = viewport.height;
const viewportScale = window.devicePixelRatio || 1;

viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;
viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';*/
// ----------------

// Percentage
const viewportWidth = viewport.clientWidth;
const viewportHeight = viewport.clientHeight;
const viewportScale = window.devicePixelRatio || 1;

viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;

viewport.style.width = "100%";
viewport.style.height = "100%";
// ----------------

const ctx = viewport.getContext("2d");
ctx.scale(viewportScale, viewportScale);

function updateGenerationCount() {
  generation_count += 1;
  let text = "Generations: " + generation_count + "\n";
  generations.innerHTML = text.replace(/(?:\r\n|\r|\n)/g, "<br>");
}

CanvasRenderingContext2D.prototype.drawBird = function(x, y, offx, offy) {
  this.fillRect(x - offx, y - offy, 2*offx, 2*offy);
  
  this.lineWidth = 3;
  this.strokeStyle = "purple";
  this.strokeRect(x - offx, y - offy, 2*offx, 2*offy);
};

CanvasRenderingContext2D.prototype.drawPipe = function(x, y, offx, offy) {
  // top
  this.fillRect(x - offx, 0, 2*offx, y - offy);
  // bottom
  this.fillRect(x - offx, y + offy, 2*offx, viewportHeight);
  // middle
  this.fillRect(x, y, 2, 2);

  this.lineWidth = 10;
  this.strokeStyle = "rgb(3, 88, 18)";
  // top
  this.strokeRect(x - offx, 0, 2*offx, y - offy);
  // bottom
  this.strokeRect(x - offx, y + offy, 2*offx, viewportHeight);

  this.lineWidth = 3;
  this.strokeStyle = "rgb(178, 247, 242)";
  this.strokeRect(x - offx, y - offy, 2*offx, 2*offy);
};

trainButton.onclick = function() {
  if(simulation.train()) {
    updateGenerationCount();
  }
}

function sleepFor(sleepDuration){
  var now = new Date().getTime();
  while(new Date().getTime() < now + sleepDuration){ 
    /* Do nothing */ 
  }
}

function redraw() {
  ctx.clearRect(0, 0, viewportWidth, viewportHeight);

  if (simulation.step()) {
    updateGenerationCount();
  }

  //sleepFor(50);

  ticks.innerHTML = "Ticks: " + simulation.ticks();

  const world = simulation.world();

  for (const pipe of world.pipes) {
    ctx.fillStyle = "rgb(11, 141, 11)";
    ctx.drawPipe(
      pipe.x * viewportWidth,
      viewportHeight - pipe.y * viewportHeight,
      pipe.offx * viewportWidth,
      pipe.offy * viewportHeight
    );
  }

  for (const bird of world.birds) {
    ctx.fillStyle = "rgb(153, 255, 255)";
    ctx.drawBird(
      bird.x * viewportWidth,
      viewportHeight - bird.y * viewportHeight,
      bird.offx * viewportWidth,
      bird.offy * viewportHeight
    );
  }

  requestAnimationFrame(redraw);
}

redraw();