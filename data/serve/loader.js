"use strict"

function load(canvas, module_path, options) {
    console.log("Loading", module_path)
    canvas.className = "loading"
    
    import(module_path)
    .then((module) => {
        module.default().then(function(obj){
            let core = new module.Core(canvas)
            core.start()
            canvas.core = core
        }).catch(function(e){
            console.error("Failed to init module:", e)
            canvas.className = "error"
        })
    }).catch(function(e) {
        console.error("Failed to load:", e)
        canvas.className = "error"
    });
}


function setup_game() {
    let main_div = document.getElementById("master")
    let options = main_div.getAttribute("options") || ""
    load(main_div, "./webpage.js", options)
}
window.onload = setup_game



