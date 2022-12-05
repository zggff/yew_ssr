let _center = [0, 0];
let _zoom = 10;
let _text;

export function set_zoom(zoom) {
    _zoom = zoom;
}
export function set_center(x, y) {
    _center = [x, y];
}

export function set_text(text) {
    _text = text
}

export function init_map() {
    if (typeof ymaps === "undefined") {
        var script = document.createElement("script");
        script.type = "text/javascript";
        script.src = "https://api-maps.yandex.ru/2.1/?lang=ru_RU&amp;apikey=75f279d9-379b-42cf-af2d-f7f5df98d242"
        script.onload = function() {
            ymaps.ready(render_map)
        }
        document.body.appendChild(script);
    }
}

export function render_map() {
    const map = new ymaps.Map("map", {
        center: _center,
        zoom: _zoom
    });
    if (typeof _text !== "undefined") {
        const points = map.geoObjects;
        points.add(new ymaps.Placemark(_center, { balloonContent: _text }, {}));

    }
}
