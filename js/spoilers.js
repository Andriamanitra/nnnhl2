for (const el of document.querySelectorAll(".spoiler")) {
    el.addEventListener("dblclick", (ev) => {
        for (const score of document.querySelectorAll(".spoiler")) {
            score.classList.remove("spoiler");
        }
    })
}
