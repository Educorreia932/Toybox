function split(str){
    const arr = [];

    for(const char of str)
      arr.push(char)
     
    return arr;
}

let request = new XMLHttpRequest();
let frames;

request.addEventListener("load", function() {
    frames = split(this.responseText);
});
request.open("get", "../out/bad_apple.txt", true);
request.send();

let frame_number = 0;

function update_frame() {
    document.getElementById("animation").innerText = frames[frame_number];
    frame_number += 1;
}

let FRAME_RATE = 30;
let PERIOD = 1000 / FRAME_RATE;

let t = setInterval(update_frame, PERIOD);