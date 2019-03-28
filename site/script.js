(function () {
    var curImage;
    var numImages;
    document.addEventListener("DOMContentLoaded", init, false);

    function init() {
        var timer = setInterval(nextImage, 4000);
        curImage = 0;
        numImages = 4;
    }

    function nextImage() {
        var img;
        // remove showImg class from current image
        img = document.getElementById("img" + curImage);
        removeClass(img, "showImg");
        
        // compute next image
        curImage++;
        if (curImage > numImages - 1) {
            curImage = 0;
        }
        
        // add showMe class to next image
        img = document.getElementById("img" + curImage);
        addClass(img, "showImg");
    }
    
    function addClass(elem, name) {
        var c = elem.className;
        if (c) c += " ";  // if not blank, add a space separator
        c += name;
        elem.className = c;
    }

    function removeClass(elem, name) {
        var c = elem.className;
        elem.className = c.replace(name, "").replace(/\s+/g, " ").replace(/^\s+|\s+$/g, "");  // remove name and extra blanks
    }

    function getRandomNumber(min, max) {
        return Math.floor(Math.random() * (max - min + 1)) + min;
    }

})();