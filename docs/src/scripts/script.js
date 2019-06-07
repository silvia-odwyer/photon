console.log('%c Hello from Photon!', 'background: lemonchiffon; border: 1px solid #fff');
console.log('%c Hire the dev behind Photon 💠', 'background: lavenderblush; border: 1px solid #000; padding: 4px; padding-top: 10px; padding-bottom: 8px;');

// add custom js below

import("../../../crate/pkg").then(module => {
    module.run();
  
    let canvas = document.getElementById("canvas");  
    let img = document.getElementById("img");
    canvas.width = img.width;
    canvas.height = img.height;
    let ctx = canvas.getContext("2d");
    ctx.drawImage(img, 0, 0);
  
    // Setup event listeners
    let hue_rotate_elem = document.getElementById("hue_rotate");
    hue_rotate_elem.addEventListener('click', function(){console.log("js_edit_time started"); console.time("js_edit_time"); editImage(canvas, ctx); console.timeEnd("js_edit_time")}, false);
  
    let saturate_elem = document.getElementById("saturate");
    saturate_elem.addEventListener('click', function(){
   
    }, false);
    ctx.drawImage(img, 0, 0);

    setTimeout(function(){applyEffect("primary"), 2000});
    setTimeout(function(){applyEffect("inc_red_channel")}, 4000)
    setTimeout(function(){applyEffect("inc_green_channel")}, 6000)
    setTimeout(function(){applyEffect("inc_blue_channel")}, 8000)
    setTimeout(function(){applyEffect("remove_red_channel")}, 10000)
    setTimeout(function(){applyEffect("remove_green_channel"), 12000});
    setTimeout(function(){applyEffect("remove_blue_channel")}, 14000)
    setTimeout(function(){applyEffect("solarize")}, 16000)
    setTimeout(function(){applyEffect("threshold")}, 18000)
    setTimeout(function(){applyEffect("sepia")}, 20000)
    setTimeout(function(){applyEffect("decompose_min"), 22000});
    setTimeout(function(){applyEffect("swap_rg_channels")}, 24000)
    setTimeout(function(){applyEffect("swap_gb_channels")}, 26000)
    setTimeout(function(){applyEffect("swap_rb_channels")}, 28000)
    setTimeout(function(){applyEffect("remove_red_channel")}, 30000)

    function applyEffect(filter_name) {
      ctx.drawImage(img, 0, 0);

      console.time("edit_time"); 

      // Get the image data from the image
      let imgData = ctx.getImageData(0, 0, canvas.width, canvas.height);

      // Convert the ImageData to a PhotonImage (so that it can communicate with the core Rust library)
      let rust_image = module.open_image(imgData, canvas.width, canvas.height);

          // Maps the name of an effect to its relevant function in the Rust library
    let filter_dict = {"grayscale" : function(){return module.grayscale(rust_image)}, 
    "offset_red": function(){return module.offset(rust_image, 0, 15)},                    
    "offset_blue": function(){return module.offset(rust_image, 1, 15)},
    "offset_green": function(){return module.offset(rust_image, 2, 15)},
    "primary" : function() {return module.primary(rust_image)},
    "solarize" : function() {return module.solarize(rust_image)},
    "threshold" : function() {return module.threshold(rust_image, 100)},
    "sepia" : function() {return module.sepia(rust_image)},
    "decompose_min" : function(){return module.decompose_min(rust_image)},
    "decompose_max" : function(){return module.decompose_max(rust_image)},
    "grayscale_shades": function(){return module.grayscale_shades(rust_image)},
    "red_channel_grayscale": function() {single_channel_grayscale(rust_image, 0)},
    "green_channel_grayscale": function() {single_channel_grayscale(rust_image, 1)},
    "blue_channel_grayscale": function() {single_channel_grayscale(rust_image, 2)},
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
    "inc_red_channel": function() {return module.alter_channel(rust_image, 0, 30)}, 
    "inc_blue_channel": function() {return module.alter_channel(rust_image, 2, 30)}, 
    "inc_green_channel": function() {return module.alter_channel(rust_image, 1, 30)}, 
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
    "sharpen": function() {return module.sharpen(rust_image)},
    "lix": function() {return module.lix(rust_image)},
    "neue": function() {return module.neue(rust_image)},
    "ryo": function() {return module.ryo(rust_image)},
    "gaussian_blur": function() {return module.gaussian_blur(rust_image)},
    "inc_brightness": function() {return module.inc_brightness(rust_image, 20)},
    "inc_lum": function() {return module.inc_luminosity(rust_image)},
    "grayscale_human_corrected": function() {return module.grayscale_human_corrected(rust_image)},
    "blend": function() {return module.blend(rust_image, rust_image2, "over")},
    "overlay": function() {return module.blend(rust_image, rust_image2, "overlay")},
    "atop": function() {return module.blend(rust_image, rust_image2, "atop")},
    "xor": function() {return module.blend(rust_image, rust_image2, "xor")},
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
    "watermark": function() {return module.watermark(rust_image, watermark_img, 10, 30)},
    "text": function() {return module.draw_text(rust_image, "welcome to WebAssembly", 10, 20)},
    "text_border": function() {return module.draw_text_with_border(rust_image, "welcome to the edge", 10, 20)},
  };

    // Filter the image, the PhotonImage's raw pixels are modified and 
    // the PhotonImage is returned
    let new_image = filter_dict[filter_name]();
    let new_pixels = module.to_image_data(new_image);
    
    // Place the pixels back on the canvas
    ctx.putImageData(new_pixels, 0, 0);

    console.timeEnd("wasm_time");

      //
  
      // let imgData = ctx.getImageData(0, 0, canvas.width, canvas.height);
      // let rust_image = module.open_image(imgData, canvas.width, canvas.height);
      // let new_pixels = module.alter_channel(rust_image, 1, 90);
      // // let new_img_data = module.inc_channel_raw(imgData, 0, 70, 3104, 4656);
      // ctx.putImageData(new_pixels, 0, 0);
      console.timeEnd("edit_time");
    }
  
  
  });
  
  function editImage(canvas, ctx) {
    let imgData = ctx.getImageData(0, 0, canvas.width, canvas.height);
    for (i = 0; i < imgData.data.length; i += 4) {
      imgData[i] += 30;
    }
    ctx.putImageData(imgData, 0, 0);
  }

