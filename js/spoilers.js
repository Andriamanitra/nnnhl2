for (const el of document.querySelectorAll(".OFF .score")) {
    el.addEventListener("dblclick", (ev) => {
        for (const score of document.querySelectorAll(".OFF .score")) {
            score.classList.remove("spoiler");
        }
    })
}
