for (const el of document.querySelectorAll('time')) {
    el.innerText = (new Date(el.dateTime)).toLocaleTimeString([], {hour: '2-digit', minute: '2-digit', hour12: false})
}