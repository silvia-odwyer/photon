console.log('%c Hello from Photon!', 'background: lemonchiffon; border: 1px solid #fff');
console.log('%c Hire the dev behind Photon -> silvi.dev', 'background: lavenderblush; border: 1px solid #000; padding: 4px; padding-top: 10px; padding-bottom: 8px;');

// add custom js below
var js_ctx, js_canvas, wasm_ctx, wasm_canvas, js_img, wasm_image;
var startTime, endTime;

import("../../pkg").then(module => {
    var canvas, ctx, img, correction_canvas, correction_ctx, img2, effects_canvas, effects_ctx, img3;

    setUpCanvases();

    let compare_wasm_elem = document.getElementById("compare_wasm");
    compare_wasm_elem.addEventListener('click', compareWASM, false);

    setUpTimeouts();
    function applyEffect(filter_name, in_canvas, in_ctx, in_img) {
      in_ctx.drawImage(in_img, 0, 0);

      // Convert the ImageData to a PhotonImage (so that it can communicate with the core Rust library)
      let rust_image = module.open_image(in_canvas, in_ctx);

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
      "swap_rg_channels": function() {return module.swap_channels(rust_image, 0, 1)}, 
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
    
    // Place the pixels back on the canvas
    module.putImageData(in_canvas, in_ctx, new_image);

    }

    function setUpComparisonImages() {
      js_img = document.getElementById("js_img");
      js_canvas = document.getElementById("js_canvas");
      js_canvas.width = js_img.width;
      js_canvas.height = js_img.height;
  
      js_ctx = js_canvas.getContext("2d");
  
      js_ctx.drawImage(js_img, 0, 0);
  
    }
    setUpComparisonImages();

  
    function updateBenchmarks(type) {
      console.log("update benchmarks");
      let time_taken = endTime - startTime;
  
      if (type == "js") {
        let time_elem = document.getElementById("js_time");
        time_elem.innerHTML = `JS Time: ${time_taken}ms`;
      }
  
      else if (type == "wasm") {
        let time_elem = document.getElementById("wasm_time");
        time_elem.innerHTML = `WASM Time: ${time_taken}ms`;
      }
    }


    function compareWASM() {
      startTime = performance.now();
      console.time("wasm_compare"); 
      js_ctx.drawImage(js_img, 0, 0);

      // Convert the ImageData to a PhotonImage (so that it can communicate with the core Rust library)
      let wasm_rust_image = module.open_image(js_canvas, js_ctx);

      // Filter the image, the PhotonImage's raw pixels are modified and 
      // the PhotonImage is returned
      let new_img = module.grayscale(wasm_rust_image); 
    
      // Place the pixels back on the canvas
      module.putImageData(js_canvas, js_ctx, new_img);
      console.timeEnd("wasm_compare");
      endTime = performance.now();
      updateBenchmarks("wasm");
    }

    function setUpCanvases() {
      canvas = document.getElementById("canvas");  
      img = document.getElementById("img");
      canvas.width = img.width;
      canvas.height = img.height;
      ctx = canvas.getContext("2d");
      ctx.drawImage(img, 0, 0);

      correction_canvas = document.getElementById("correction_canvas");
      img2 = document.getElementById("img2");
      correction_canvas.width = img2.width;
      correction_canvas.height = img2.height;
      correction_ctx = correction_canvas.getContext("2d");
      correction_ctx.drawImage(img2, 0, 0);

      img3 = document.getElementById("img3");
      effects_canvas = document.getElementById("effects_canvas");
      effects_canvas.width = img3.width;
      effects_canvas.height = img3.height;
      effects_ctx = effects_canvas.getContext("2d");
      effects_ctx.drawImage(img3, 0, 0);
     
    }

    function setUpTimeouts() {
      let landing_effects = ["primary", "inc_red_channel", "inc_blue_channel", "remove_red_channel", 
      "solarize", "threshold", "sepia", "decompose_min", "swap_rg_channels", "swap_gb_channels", 
      "remove_red_channel", "dec_green_channel", "dec_blue_channel", "dec_red_channel", "emboss", "sharpen"];
  
      var timer = 2000;
      var time_dec = 0;
      for (let k = 0; k < landing_effects.length; k++) {
        let effect = landing_effects[k];
        setTimeout(function(){applyEffect(effect, canvas, ctx, img)}, timer);
        timer += 1000;
        timer -= time_dec;
        if (time_dec > 900) {
          time_dec = 0;
        }
        time_dec += 100;
      }

      let correction_effects = ["inc_red_channel", "inc_blue_channel", "sepia", "swap_rg_channels", "swap_gb_channels", "dec_green_channel", "dec_blue_channel", "dec_red_channel"];
  
      setInterval(function() {
      timer = 2000;
      time_dec = 0;
      for (let k = 0; k < correction_effects.length; k++) {
        let effect = correction_effects[k];
        setTimeout(function(){applyEffect(effect, correction_canvas, correction_ctx, img2)}, timer);
        timer += 1000;
      }
  
    }, 10000);

    let effects = ["primary", "emboss", "sharpen"];
    setInterval(function() {
      timer = 1000;
      time_dec = 0;
      for (let k = 0; k < effects.length; k++) {
        let effect = effects[k];
        setTimeout(function(){applyEffect(effect, effects_canvas, effects_ctx, img3)}, timer);
        timer += 500;
      }
    }, 4000);
  
  }
});

  // 45 degree change for imagery.
  let c = 45;

function draw(){
  document.documentElement.style.setProperty('--direction', c++ + 'deg');
  requestAnimationFrame(draw);
}

requestAnimationFrame(draw);