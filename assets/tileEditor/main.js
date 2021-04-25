images = ["", "tileset.png"];

var map = [];
var boolMap = [];

// 0 1 2 3
// 4 5 6 7
// 8 9 1011
// 12131415

// up right down left
var types = [
    [0, 0, 0, 0, 15],
    [0, 0, 0, 1, 14],
    [0, 0, 1, 0, 3],
    [0, 0, 1, 1, 2],
    [0, 1, 0, 0, 12],
    [0, 1, 0, 1, 13],
    [0, 1, 1, 0, 0],
    [0, 1, 1, 1, 1],
    [1, 0, 0, 0, 11],
    [1, 0, 0, 1, 10],
    [1, 0, 1, 0, 7],
    [1, 0, 1, 1, 6],
    [1, 1, 0, 0, 8],
    [1, 1, 0, 1, 9],
    [1, 1, 1, 0, 4],
    [1, 1, 1, 1, 5]
];

var brushSize = 1;

var collisions = [];

for (var y = 0; y < 216; y++) {
    var arr = [];
    var arr2 = [];
    for (var x = 0; x < 72; x++) {
        arr.push(0);
        arr2.push(false);
    }
    map.push(arr);
    boolMap.push(arr2);
}

setup(60);

function onAssetsLoaded() {
    for (var y = 0; y < 4; y++) {
        for (var x = 0; x < 4; x++) {
            var newCanv = document.createElement("canvas");
            newCanv.width = 10;
            newCanv.height = 10;
            var newCtx = newCanv.getContext("2d");
            newCtx.drawImage(sprites.tileset.spr, x * 10, y * 10, 10, 10, 0, 0, 10, 10);
            sprites[x + y * 4] = { spr: newCanv, drawLimitSize: 10 };
        }
    }
    parseMap();
}

function update() {}

function input() {
    if (keyDown[k.w]) {
        moveCamera(0, -5);
    }
    if (keyDown[k.s]) {
        moveCamera(0, 5);
    }
    if (keyDown[k.a]) {
        moveCamera(-5, 0);
    }
    if (keyDown[k.d]) {
        moveCamera(5, 0);
    }

    if (keyPress[k.q]) {
        if (brushSize > 1) {
            brushSize--;
        }
    }
    if (keyPress[k.e]) {
        brushSize++;
    }

    camera.zoom += scroll;
    if (camera.zoom < 1) {
        camera.zoom = 1;
    }

    if (keyPress[k.ENTER]) {
        renderMap();
    }

    if (keyPress[k.SPACE]) {
        collisions = makeOptimizedCollision();
    }

    var x = Math.floor(mousePosition().x / 10);
    var y = Math.floor(mousePosition().y / 10);
    x = x < 0 ? 0 : x;
    y = y < 0 ? 0 : y;
    x = x > map[0].length - 1 ? map[0].length - 1 : x;
    y = y > map.length - 1 ? map.length - 1 : y;
    if (mouseDown[0]) {
        for (var yy = -brushSize / 2; yy < brushSize / 2; yy++) {
            for (var xx = -brushSize / 2; xx < brushSize / 2; xx++) {
                var posX = ~~(1 + xx + x);
                var posY = ~~(1 + yy + y);
                if (posX > -1 && posY > -1 && posY < map.length && posX < map[0].length) {
                    boolMap[posY][posX] = true;
                }
            }
        }
    }
    if (mouseDown[2]) {
        for (var yy = -brushSize / 2; yy < brushSize / 2; yy++) {
            for (var xx = -brushSize / 2; xx < brushSize / 2; xx++) {
                var posX = ~~(1 + xx + x);
                var posY = ~~(1 + yy + y);
                if (posX > -1 && posY > -1 && posY < map.length && posX < map[0].length) {
                    boolMap[posY][posX] = false;
                }
            }
        }
    }
}

function draw() {
    parseMap();
    var w = map[0].length * 10;
    var h = map.length * 10;
    rect(w / 2, h / 2, w, h, "blue");
    for (var y = 0; y < map.length; y++) {
        for (var x = 0; x < map[0].length; x++) {
            if (map[y][x] !== -1) {
                img(sprites[map[y][x]], x * 10 + 5, y * 10 + 5);
            }
        }
    }

    for(var i=0;i<collisions.length;i++) {
        rectOut(collisions[i].x,collisions[i].y,collisions[i].width,collisions[i].height,"#ff0000");
    }
}

function renderMap() {
    var renderCvs = document.createElement("canvas");
    renderCvs.width = map[0].length * 10;
    renderCvs.height = map.length * 10;
    var renderCtx = renderCvs.getContext("2d");
    camera.x = 0;
    camera.y = 0;
    drawMode = 0;
    absDraw = true;
    curCtx = renderCtx;
    for (var y = 0; y < map.length; y++) {
        for (var x = 0; x < map[0].length; x++) {
            if (map[y][x] !== -1) {
                img(sprites[map[y][x]], x * 10 + 5, y * 10 + 5);

                if (map[y][x] === 5) {
                    var closest = 6;
                    for (var y2 = -4; y2 < 5; y2++) {
                        for (var x2 = -4; x2 < 5; x2++) {
                            if(y2 === 0 && x2 === 0) {
                                continue;
                            }
                            var posX = x + x2;
                            var posY = y + y2;
                            if (posX > -1 && posY > -1 && posY < map.length && posX < map[0].length) {
                                if (boolMap[posY][posX] === false) {
                                    var distance = dist({ x: x, y: y }, { x: posX, y: posY });
                                    if (distance < closest) {
                                        closest = distance;
                                    }
                                }
                            }
                        }
                    }
                    if (closest > 1) {
                        rect(x * 10 + 5, y * 10 + 5, 10, 10, `#000000${Math.round(map_range(closest, 1, 6, 50, 255)).toString(16)}`);
                    }
                }
            }
        }
    }
    document.body.appendChild(renderCvs);

    collisions = makeOptimizedCollision();

    var txt = document.createElement("textarea");
    txt.innerText = JSON.stringify(collisions);
    document.body.appendChild(txt);
}

function map_range(value, low1, high1, low2, high2) {
    return low2 + ((high2 - low2) * (value - low1)) / (high1 - low1);
}

function parseMap() {
    for (var y = 0; y < map.length; y++) {
        for (var x = 0; x < map[0].length; x++) {
            if (boolMap[y][x] === false) {
                map[y][x] = -1;
                continue;
            }
            var t = 0,
                b = 0,
                l = 0,
                r = 0;
            if (y === 0) {
                t = 1;
            } else {
                t = boolMap[y - 1][x] ? 1 : 0;
            }
            if (x === 0) {
                l = 1;
            } else {
                l = boolMap[y][x - 1] ? 1 : 0;
            }
            if (y === map.length - 1) {
                b = 1;
            } else {
                b = boolMap[y + 1][x] ? 1 : 0;
            }
            if (x === map[0].length - 1) {
                r = 1;
            } else {
                r = boolMap[y][x + 1] ? 1 : 0;
            }
            for (var i = 0; i < 16; i++) {
                if (types[i][0] === t && types[i][1] === r && types[i][2] === b && types[i][3] === l) {
                    map[y][x] = types[i][4];
                    break;
                }
            }
        }
    }
}


function makeOptimizedCollision() {
    var tileCount = 0;
    var worldW = map[0].length;
    var worldH = map.length;
    cols = [];

    // 2d array of booleans for if a tile has gotten a collision made for it
    var hasCollision = [];

    // fill arrays
    for (var y = 0; y < worldH; y++) {
        var hasCollisionRow = [];
        for (var x = 0; x < worldW; x++) {
            hasCollisionRow.push(false);
        }
        hasCollision.push(hasCollisionRow);
    }

    // try to make large rectangles that cover multiple walls to make collision more efficient

    for (var y = 0; y < worldH; y++) {
        for (var x = 0; x < worldW; x++) {
            if (!hasCollision[y][x] && boolMap[y][x]) {
                // find right limit
                var xPos = x;
                while (xPos < worldW && boolMap[y][xPos]) {
                    xPos++;
                }
                xPos--;

                // find bottom limit
                var yPos = y;
                var fullRow = true;
                // go down row by row
                while (yPos < worldH-1 && boolMap[yPos][xPos] && fullRow) {
                    yPos++;
                    // go through the whole row, make sure it is full
                    var rowX = xPos;
                    while (rowX > -1 && boolMap[yPos][rowX]) {
                        rowX--;
                    }
                    // if the row is not full, stop
                    if (rowX + 1 !== x) {
                        fullRow = false;
                        yPos--;
                    }
                }

                // track what tiles have gotten collision
                for (var y2 = y; y2 < yPos + 1; y2++) {
                    for (var x2 = x; x2 < xPos + 1; x2++) {
                        hasCollision[y2][x2] = true;
                        tileCount++;
                    }
                }

                // find collider dimensions
                var colX = (x + xPos + 1) / 2;
                var colY = (y + yPos + 1) / 2;
                var colW = xPos - x + 1;
                var colH = yPos - y + 1;

                // add collider
                cols.push({ x: colX * 10, y: colY * 10, width: colW * 10, height: colH * 10 });
            }
        }
    }

    console.log(`tiles: ${tileCount}, boxes: ${cols.length}`)
    return cols;
}