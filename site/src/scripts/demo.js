import Fruit from "../images/nine_yards.jpg";
import Daisies from "../images/nine_yards.jpg";
import Underground from "../images/underground.jpg";
import NineYards from "../images/nine_yards.jpg";
import BlueMetro from "../images/blue_metro.jpg";
import Watermark from "../images/wasm_logo.png";


// Setup global variables
var canvas, canvas2, watermark_canvas;
var ctx, ctx2, watermark_ctx;
var newimg, watermark_img, img2;

 import("@silvia-odwyer/photon").then(module => {
  var startTime;
  var endTime;

  setUpEventListeners();

   function setUpEventListeners() {
    document.getElementById('img_uploader').addEventListener('change', readURL, true);
     // Setup event listeners

     let resize_btns = document.getElementsByClassName("resize");
     for (let i = 0; i < resize_btns.length; i++) {
      let button = resize_btns[i];
      button.addEventListener("click", function(){resize(event)}, false);
    }

    let crop_btn = document.getElementById("crop");

    crop_btn.addEventListener("click", function(){crop(event)}, false);
   
     let filter_buttons = document.getElementsByClassName("filter");
     for (let i = 0; i < filter_buttons.length; i++) {
       let button = filter_buttons[i];
       button.addEventListener("click", function(){filterImage(event)}, false);
     }

     let effect_buttons = document.getElementsByClassName("effect");
     for (let i = 0; i < effect_buttons.length; i++) {
       let button = effect_buttons[i];
       button.addEventListener("click", function(){applyEffect(event)}, false);
     }

     let blend_buttons = document.getElementsByClassName("blend");
     for (let i = 0; i < blend_buttons.length; i++) {
       let button = blend_buttons[i];
       button.addEventListener("click", function(){blendImages(event)}, false);
     }

     let overlay_buttons = document.getElementsByClassName("overlay");		
     for (let i = 0; i < overlay_buttons.length; i++) {		
       let button = overlay_buttons[i];		
       button.addEventListener("click", function(){watermark(event)}, false);		
     }
     setUpImages();
   }

   function watermark() {
    startTime = performance.now();
    setUpWatermark();

    let src_canvas = document.getElementById("canvas");
    let src_photon_img = module.open_image(canvas, ctx)
    let watermark_photon_img = module.open_image(watermark_canvas, watermark_ctx);

    console.log(watermark_photon_img)
    console.log(src_photon_img)

    module.watermark(src_photon_img, watermark_photon_img, 30, 40);  
    module.putImageData(canvas, ctx, src_photon_img);		
    endTime = performance.now();

    updateBenchmarks();
   }

   function watermark_js() {
    startTime = performance.now();

    for (let k = 0; k < 1500; k++) {
      let img = document.getElementById("img");

      ctx.drawImage(img, 50, 70);
    }

    endTime = performance.now();

    updateBenchmarks();
  }

   function blendImages(event) {
     console.time("wasm_blend_time"); 

     ctx.drawImage(newimg, 0, 0);
     startTime = performance.now();

     // Get the name of the effect the user wishes to apply to the image
     // This is the id of the element they clicked on
     let filter_name = event.target.id;

     // Create a PhotonImage from the canvas + context (so that it can communicate with the core Rust library)
     let rust_image = module.open_image(canvas, ctx);

     // Create a PhotonImage from 2nd image to be blended with the 1st.
     let rust_image2 = module.open_image(canvas2, ctx2);

     let watermark_img = module.open_image(watermark_canvas, watermark_ctx);
    
    //  Maps the name of an effect to its relevant function in the Rust library
     let filter_dict = {
                       "blend": function() {return module.blend(rust_image, rust_image2, "over")},
                       "overlay": function() {return module.blend(rust_image, rust_image2, "overlay")},
                       "atop": function() {return module.blend(rust_image, rust_image2, "atop")},
                       "plus": function() {return module.blend(rust_image, rust_image2, "plus")},
                       "multiply": function() {return module.blend(rust_image, rust_image2, "multiply")},
                       "burn": function() {return module.blend(rust_image, rust_image2, "burn")},
                       "difference": function() {return module.blend(rust_image, rust_image2, "difference")},
                       "soft_light": function() {return module.blend(rust_image, rust_image2, "soft_light")},
                       "hard_light": function() {return module.blend(rust_image, rust_image2, "hard_light")},
                       "dodge": function() {return module.blend(rust_image, rust_image2, "dodge")},
                       "exclusion": function() {return module.blend(rust_image, rust_image2, "exclusion")},
                       "lighten": function() {return module.blend(rust_image, rust_image2, "lighten")},
                       "darken": function() {return module.blend(rust_image, rust_image2, "darken")},
                       "watermark": function() {return module.watermark(canvas, watermark_canvas, 10, 30)},
                     };

     // Filter the image, the PhotonImage's raw pixels are modified and 
     // the PhotonImage is returned
     filter_dict[filter_name]();

     // Update the canvas with the new imagedata
     module.putImageData(canvas, ctx, rust_image);
     console.timeEnd("wasm_blend_time");
     endTime = performance.now()
     updateBenchmarks()
   }

   function colour_space(rust_image, colour_space, effect) {
     if (colour_space == "hsl") {
       module.lch(rust_image, effect, 0.3);
     }
     else if (colour_space == "hsv") {
       module.hsv(rust_image, effect, 0.3)
     }
     else {
       module.lch(rust_image, effect, 0.3);
     }
     updateCanvas(rust_image);
   }
  
   function updateCanvas(new_image) {
     let new_pixels = module.to_image_data(new_image);
    
    // Place the pixels back on the canvas
     ctx.putImageData(new_pixels, 0, 0);
   }

   function vec_to_photonimage_example() {
     console.time("vec_wasm_time"); 

     ctx.drawImage(newimg, 0, 0);
     startTime = performance.now();

     let base64 = canvas.toDataURL();
     base64 = base64.substr(22, base64.length);
    
    // Convert the raw base64 data to a Vec of u8s.
    let vec = module.base64_to_vec(base64);
    
    // Convert the Vec of u8s to a PhotonImage
    let photon_img = module.photonimage_from_vec(vec); 
    module.grayscale(photon_img);

    // Update the canvas with the new imagedata
     module.putImageData(canvas, ctx, photon_img);
     console.timeEnd("vec_wasm_time");
     endTime = performance.now();
     updateBenchmarks();
   }

   function readURL() {
     let file = document.getElementById("img_uploader").files[0];

     let reader = new FileReader();
     reader.onloadend = function () {
         newimg.src = reader.result;  // Set the global image to the path of the file on the client's PC.
     }
     if (file) {
         reader.readAsDataURL(file);
     } else {
         // Error message TODO
         console.log("Could not read file. :(")
     }
   }

   function base64_example() {

     ctx.drawImage(newimg, 0, 0);
     startTime = performance.now();

     let base64 = canvas.toDataURL();
     base64 = base64.substr(22, base64.length);
    
     // Convert the raw base64 data to a PhotonImage.
     console.time("base64_wasm_time"); 
     let photon_img = module.base64_to_image(base64);

     module.grayscale(photon_img);

     // Update the canvas with the new imagedata
     module.putImageData(canvas, ctx, photon_img);
     console.timeEnd("base64_wasm_time");
     endTime = performance.now();
     updateBenchmarks();
     updateEffectName(event.target);
   }

   function filterImage(event) {
     startTime = performance.now();
     ctx.drawImage(newimg, 0, 0);
     let filter_name = event.target.id;
  
     console.time("wasm_time"); 

     // Create a PhotonImage from the canvas + context (so that it can communicate with the core Rust library)
     let rust_image = module.open_image(canvas, ctx);

    // Filter the image, the PhotonImage's raw pixels are modified and 
    // the PhotonImage is returned
     module.filter(rust_image, filter_name);
    
     endTime = performance.now();
     updateBenchmarks();
     // Place the pixels back on the canvas
     module.putImageData(canvas, ctx, rust_image);
     console.timeEnd("wasm_time");
   }

   function crop() {
      startTime = performance.now();
      let cropped_canvas = module.crop_img_browser(canvas, 300, 500, 40, 40);
      let resized_img_container = document.getElementById("resized_imgs");
      resized_img_container.appendChild(cropped_canvas);
      endTime = performance.now()
      updateBenchmarks()
   }

   function resize(event) {
     let resize_type = event.target.id;
     resize_type = Number(resize_type);
     
     ctx.drawImage(newimg, 0, 0);
     startTime = performance.now();
     let resized_img_container = document.getElementById("resized_imgs");
     let photon_img = module.open_image(canvas, ctx);
 
     let newcanvas = module.resize_img_browser(photon_img, 360, 239, resize_type);
    //  let ctx2 = newcanvas.getContext("2d");
    //  let photon_img3 = module.open_image(newcanvas, ctx2);
    //  module.sharpen(photon_img3);

    //  let final_canvas = document.createElement("canvas");
    //  final_canvas.width = newcanvas.width;
    //  final_canvas.height = newcanvas.height;

    //  let final_ctx = final_canvas.getContext("2d");
    //   final_ctx.putImageData(photon_img3.get_image_data(), 0, 0);
     resized_img_container.appendChild(newcanvas);

     endTime = performance.now();
     updateBenchmarks();
   }
  
   function applyEffect(event) {
     // Reset canvas by re-drawing the image onto the canvas
     ctx.drawImage(newimg, 0, 0);
     startTime = performance.now();

     // Get the name of the effect the user wishes to apply to the image
     // This is the id of the element they clicked on
     let filter_name = event.target.id;
    
     // Convert the ImageData to a PhotonImage (so that it can communicate with the core Rust library)
     let rust_image = module.open_image(canvas, ctx);

     // Maps the name of an effect to its relevant function in the Rust library
     let filter_dict = {"grayscale" : function(){return module.grayscale(rust_image)}, 
                       "offset_red": function(){return module.offset(rust_image, 0, 25)},                    
                       "offset_blue": function(){return module.offset(rust_image, 1, 25)},
                       "offset_green": function(){return module.offset(rust_image, 2, 25)},
                       "primary" : function() {return module.primary(rust_image)},
                       "solarize" : function() {return module.solarize(rust_image)},
                       "oil" : function() {return module.oil(rust_image,4,55.0)},
                       "threshold" : function() {return module.threshold(rust_image, 100)},
                       "sepia" : function() {return module.sepia(rust_image)},
                       "decompose_min" : function(){return module.decompose_min(rust_image)},
                       "decompose_max" : function(){return module.decompose_max(rust_image)},
                       "grayscale_shades": function(){return module.grayscale_shades(rust_image, 8)},
                       "red_channel_grayscale": function() {return module.single_channel_grayscale(rust_image, 0)},
                       "green_channel_grayscale": function() {return module.single_channel_grayscale(rust_image, 1)},
                       "blue_channel_grayscale": function() {return module.single_channel_grayscale(rust_image, 2)},
                       "hue_rotate_hsl": function() {colour_space(rust_image, "hsl", "shift_hue")}, 
                       "hue_rotate_hsv": function() {colour_space(rust_image, "hsv", "shift_hue")}, 
                       "hue_rotate_lch": function() {colour_space(rust_image, "lch", "shift_hue")}, 
                       "lighten_hsl": function() {colour_space(rust_image, "hsl", "lighten")}, 
                       "lighten_hsv": function() {colour_space(rust_image, "hsv", "lighten")}, 
                       "lighten_lch": function() {colour_space(rust_image, "lch", "lighten")}, 
                       "darken_hsl": function() {colour_space(rust_image, "hsl", "darken")}, 
                       "darken_hsv": function() {colour_space(rust_image, "hsv", "darken")}, 
                       "darken_lch": function() {colour_space(rust_image, "lch", "darken")}, 
                       "desaturate_hsl": function() {colour_space(rust_image, "hsl", "desaturate")}, 
                       "desaturate_hsv": function() {colour_space(rust_image, "hsv", "desaturate")}, 
                       "desaturate_lch": function() {colour_space(rust_image, "lch", "desaturate")}, 
                       "saturate_hsl": function() {colour_space(rust_image, "hsl", "saturate")}, 
                       "saturate_hsv": function() {colour_space(rust_image, "hsv", "saturate")}, 
                       "saturate_lch": function() {colour_space(rust_image, "lch", "saturate")}, 
                       "inc_red_channel": function() {return module.alter_channel(rust_image, 0, 90)}, 
                       "inc_blue_channel": function() {return module.alter_channel(rust_image, 2, 90)}, 
                       "inc_green_channel": function() {return module.alter_channel(rust_image, 1, 90)}, 
                       "inc_two_channels": function() {return module.alter_channel(rust_image, 1, 30);}, 
                       "dec_red_channel": function() {return module.alter_channel(rust_image, 0, -30)}, 
                       "dec_blue_channel": function() {return module.alter_channel(rust_image, 2, -30)}, 
                       "dec_green_channel": function() {return module.alter_channel(rust_image, 1, -30)}, 
                       "swap_rg_channels": function() {return module.swap_channels(rust_image, 0, 1);}, 
                       "swap_rb_channels": function() {return module.swap_channels(rust_image, 0, 2);}, 
                       "swap_gb_channels": function() {return module.swap_channels(rust_image, 1, 2);}, 
                       "remove_red_channel": function() {return module.remove_red_channel(rust_image, 250);}, 
                       "remove_green_channel": function() {return module.remove_green_channel(rust_image, 250)},
                       "remove_blue_channel": function() {return module.remove_blue_channel(rust_image, 250)},
                       "emboss": function() {return module.emboss(rust_image)},
                       "box_blur": function() {return module.box_blur(rust_image)},
                       "gradient": function() { return module.apply_gradient(rust_image)},
                       "sharpen": function() {return module.sharpen(rust_image)},
                       "duotone": function() {return module.duotone(rust_image, rgb1, rgb2)},
                       "lix": function() {return module.lix(rust_image)},
                       "neue": function() {return module.neue(rust_image)},
                       "ryo": function() {return module.ryo(rust_image)},
                       "gaussian_blur": function() {return module.gaussian_blur(rust_image)},
                       "horizontal_strips": function() { return module.horizontal_strips(rust_image, 6)},
                       "vertical_strips": function() { return module.vertical_strips(rust_image, 6)},
                       "inc_brightness": function() {return module.inc_brightness(rust_image, 20)},
                       "inc_lum": function() {return module.inc_luminosity(rust_image)},
                       "grayscale_human_corrected": function() {return module.grayscale_human_corrected(rust_image)},
                       "watermark": function() {return module.watermark(rust_image, watermark_img, 10, 30)},
                       "flipv": function() {return module.flipv(rust_image)}, 
                       "fliph": function() {return module.fliph(rust_image)},
                       "sobel_horizontal": function() {return module.sobel_horizontal(rust_image)},
                       "sobel_vertical": function() {return module.sobel_vertical(rust_image)},
                       "laplace": function() {return module.laplace(rust_image)},
                       "prewitt": function() {return module.prewitt_horizontal(rust_image)},
                       "noise_reduction": function() {return module.noise_reduction(rust_image)},
                       "identity": function() {return module.identity(rust_image)},
                       "edge_one": function() {return module.edge_one(rust_image)},
                       "edge_detection": function() {return module.edge_detection(rust_image)},
                       "lofi": function() {return module.lofi(rust_image)},
                       "cali": function() {return module.cali(rust_image)},
                       "obsidian": function() {return module.obsidian(rust_image)},
                       "firenze": function() {return module.firenze(rust_image)},
                       "pastel_pink": function() {return module.pastel_pink(rust_image)},
                       "dramatic": function() {return module.dramatic(rust_image)},
                       "golden": function() {return module.golden(rust_image)},

                     
                     
                      };
                  
    // Update the canvas with the new imagedata

    // Filter the image, the PhotonImage's raw pixels are modified and 
    // the PhotonImage is returned
    filter_dict[filter_name]();
    module.putImageData(canvas, ctx, rust_image);

     endTime = performance.now()
     updateBenchmarks()
   }

   function overlayImage(event) {		  
     console.time("wasm_time"); 		
    
     // Reset canvas by re-drawing the image onto the canvas		
     ctx.drawImage(newimg, 0, 0);		
     startTime = performance.now();		
    
     // Get the name of the effect the user wishes to apply to the image		
     // This is the id of the element they clicked on		
     let filter_name = event.target.id;		
        
     // Convert the ImageData to a PhotonImage (so that it can communicate with the core Rust library)		
     let rust_image = module.open_image(canvas, ctx);		
    
     let watermark_img = module.open_image(watermark_canvas, watermark_ctx);		
    
     // Maps the name of an effect to its relevant function in the Rust library		
     let filter_dict = {"watermark": function() {return module.watermark(rust_image, watermark_img, 10, 30)}};		
    
     // Filter the image, the PhotonImage's raw pixels are modified and 		
     // the PhotonImage is returned		
     filter_dict[filter_name]();		
    
     // Update the canvas with the new imagedata		
     module.putImageData(canvas, ctx, rust_image);		
     console.timeEnd("wasm_time");		
     endTime = performance.now()		
     updateBenchmarks()		
   }

  

  function setUpCanvas() {
    let element = document.getElementById("image_container");
    element.appendChild(newimg);

    canvas = document.getElementById("canvas");
    canvas.width = newimg.width;
    canvas.height = newimg.height;

    ctx = canvas.getContext("2d");
    ctx.drawImage(newimg, 0, 0);
  }

  // Canvas2 contains the 2nd image for blending with the first image.
  function setUpCanvas2() {
    let element = document.getElementById("image_container");
    element.appendChild(img2);
    canvas2 = document.createElement("canvas");
    canvas2.width = img2.width;
    canvas2.height = img2.width;

    ctx2 = canvas2.getContext("2d");
    ctx2.drawImage(img2, 0, 0);

   }

  function setUpWatermark() {
    let element = document.getElementById("image_container");
    element.appendChild(watermark_img);
    watermark_canvas = document.createElement("canvas");
    watermark_canvas.width = watermark_img.width;
    watermark_canvas.height = watermark_img.height;

    watermark_ctx = watermark_canvas.getContext("2d");
    watermark_ctx.drawImage(watermark_img, 0, 0);

 }

  function updateBenchmarks() {
    console.log("update benchmarks");
    let time_taken = endTime - startTime;
    let time_elem = document.getElementById("time");
    time_elem.innerHTML = `Time: ${time_taken.toFixed(2)}ms`;
  }

   function setUpImages() {
       //  Setup images
       newimg = new Image();
       newimg.src = Fruit;
      newimg.style.display = "none";
      newimg.onload = () => {
        setUpCanvas();
      }

      img2 = new Image();
      img2.src = Daisies;
      img2.style.display = "none";
      img2.onload=() => {
        setUpCanvas2();
      }

      watermark_img = new Image();
      watermark_img.src = Watermark;
     watermark_img.style.display = "none";
     watermark_img.onload = () => {
       setUpWatermark();
     }
   // Change the image currently being edited.
   let change_image_elems = document.getElementsByClassName("change_image");

    for (let i = 0; i < change_image_elems.length; i++) {
      let change_image_elem = change_image_elems[i];
      change_image_elem.addEventListener("click", function(event) {
        console.log("image changed")
        let img_name = event.target.id;
        let imgNamesToImages = {"underground": Underground, "blue_metro": BlueMetro, "nine_yards": NineYards, "daisies": Daisies, "fruit": Fruit};
        newimg.src = imgNamesToImages[img_name];
        newimg.onload = () => {
          canvas.width = newimg.width;
          canvas.height = newimg.height;
          ctx.drawImage(newimg, 0, 0);
        }
      }, false);
    }
  }

 });

 function editImage(canvas, ctx) {
   let imgData = ctx.getImageData(0, 0, canvas.width, canvas.height);
   for (let i = 0; i < imgData.data.length; i += 4) {
     imgData[i] += 30;
   }
   ctx.putImageData(imgData, 0, 0);
 }
