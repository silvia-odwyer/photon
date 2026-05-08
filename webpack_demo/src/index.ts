import MainImage from "./assets/nine_yards.jpg";
import Underground from "./assets/underground.jpg";
import NineYards from "./assets/nine_yards.jpg";
import BlueMetro from "./assets/blue_metro.jpg";
import Watermark from "./assets/wasm_logo.png";

type PhotonModule = typeof import("@silvia-odwyer/photon");
type ExtendedPhotonModule = PhotonModule &
	Record<string, (...args: unknown[]) => unknown>;
type WasmAssetModule = { default: string };

type Timing = {
	start: number;
	end: number;
};

const timing: Timing = {
	start: 0,
	end: 0,
};

let canvas: HTMLCanvasElement;
let canvas2: HTMLCanvasElement;
let watermarkCanvas: HTMLCanvasElement;
let ctx: CanvasRenderingContext2D;
let ctx2: CanvasRenderingContext2D;
let watermarkCtx: CanvasRenderingContext2D;
let newimg: HTMLImageElement;
let img2: HTMLImageElement;
let watermarkImg: HTMLImageElement;

const loadPhoton = async (): Promise<ExtendedPhotonModule> => {
	const [module, wasmModule] = await Promise.all([
		import("@silvia-odwyer/photon"),
		import(
			"@silvia-odwyer/photon/photon_rs_bg.wasm?url"
		) as Promise<WasmAssetModule>,
	]);

	const init = module.default as
		| ((moduleOrPath?: unknown) => Promise<unknown>)
		| undefined;

	if (typeof init === "function") {
		await init(wasmModule.default);
	}
	return module as ExtendedPhotonModule;
};

const bootstrap = async () => {
	let photon: ExtendedPhotonModule;
	try {
		photon = await loadPhoton();
	} catch (error) {
		console.error("Failed to initialise Photon WASM module", error);
		return;
	}

	const newImage = new Image();
	newImage.src = MainImage;
	newImage.style.display = "none";
	newImage.onload = () => {
		newimg = newImage;
		setUpCanvas();
		ensureBlendCanvasSize();
	};

	const overlayImage = new Image();
	overlayImage.src = NineYards;
	overlayImage.style.display = "none";
	overlayImage.onload = () => {
		img2 = overlayImage;
		setUpCanvas2();
		ensureBlendCanvasSize();
	};

	const watermarkImage = new Image();
	watermarkImage.src = Watermark;
	watermarkImage.style.display = "none";
	watermarkImage.onload = () => {
		watermarkImg = watermarkImage;
		setUpWatermark();
	};

	const hueRotateElem = document.getElementById("hue_rotate");
	hueRotateElem?.addEventListener("click", () => {
		console.time("js_edit_time");
		editImage(canvas, ctx);
		console.timeEnd("js_edit_time");
	});

	const filterButtons = document.getElementsByClassName("filter");
	for (let i = 0; i < filterButtons.length; i += 1) {
		const button = filterButtons[i] as HTMLElement;
		button.addEventListener("click", (event) => filterImage(event, photon));
	}

	const effectButtons = document.getElementsByClassName("effect");
	for (let i = 0; i < effectButtons.length; i += 1) {
		const button = effectButtons[i] as HTMLElement;
		button.addEventListener("click", (event) => applyEffect(event, photon));
	}

	const noiseButtons = document.getElementsByClassName("noise");
	for (let i = 0; i < noiseButtons.length; i += 1) {
		const button = noiseButtons[i] as HTMLElement;
		button.addEventListener("click", (event) => applyEffect(event, photon));
	}

	const blendButtons = document.getElementsByClassName("blend");
	for (let i = 0; i < blendButtons.length; i += 1) {
		const button = blendButtons[i] as HTMLElement;
		button.addEventListener("click", (event) => blendImages(event, photon));
	}

	const resizeBtn = document.getElementById("resize");
	resizeBtn?.addEventListener("click", (event) => resize(event, photon));

	const changeImageBtn = document.getElementById("change_img");
	changeImageBtn?.addEventListener("click", changeImageFromNav);

	const changeImageElems = document.getElementsByClassName("change_image");
	for (let i = 0; i < changeImageElems.length; i += 1) {
		const changeImageElem = changeImageElems[i] as HTMLElement;
		changeImageElem.addEventListener("click", changeImage);
	}
};

const ensureContexts = (): boolean => {
	if (!canvas || !ctx || !newimg) {
		console.warn("Photon demo is still initialising.");
		return false;
	}
	return true;
};

const ensureBlendCanvasSize = () => {
	if (!canvas2 || !ctx2 || !newimg || !img2) {
		return;
	}
	const targetWidth = newimg.width >= img2.width ? newimg.width + 1 : img2.width;
	const targetHeight =
		newimg.height >= img2.height ? newimg.height + 1 : img2.height;
	if (canvas2.width !== targetWidth || canvas2.height !== targetHeight) {
		canvas2.width = targetWidth;
		canvas2.height = targetHeight;
	}
	ctx2.clearRect(0, 0, canvas2.width, canvas2.height);
	ctx2.drawImage(img2, 0, 0, canvas2.width, canvas2.height);
};

const blendWithMode = (
	photon: ExtendedPhotonModule,
	base: ReturnType<ExtendedPhotonModule["open_image"]>,
	overlay: ReturnType<ExtendedPhotonModule["open_image"]>,
	mode: string,
) => {
	if (!overlay) {
		return;
	}
	try {
		photon.blend(base, overlay, mode);
	} catch (error) {
		console.warn(`Blend mode "${mode}" failed`, error);
	}
};

const filterImage = (event: Event, photon: ExtendedPhotonModule) => {
	if (!ensureContexts()) {
		return;
	}
	const target = event.target as HTMLElement | null;
	if (!target) {
		return;
	}
	timing.start = performance.now();
	ctx.drawImage(newimg, 0, 0);
	const filterName = target.id;

	console.time("wasm_time");

	const rustImage = photon.open_image(canvas, ctx);
	photon.filter(rustImage, filterName);
	photon.putImageData(canvas, ctx, rustImage);

	timing.end = performance.now();
	updateBenchmarks();
	updateEffectName(target);
	console.timeEnd("wasm_time");
};

const applyEffect = (event: Event, photon: ExtendedPhotonModule) => {
	if (!ensureContexts() || !watermarkCanvas || !watermarkCtx) {
		return;
	}
	console.time("wasm_time");

	ctx.drawImage(newimg, 0, 0);
	timing.start = performance.now();

	const target = event.target as HTMLElement | null;
	if (!target) {
		return;
	}
	const filterName = target.id;
	const rustImage = photon.open_image(canvas, ctx);
	// const rustImage2 = canvas2 && ctx2 ? photon.open_image(canvas2, ctx2) : null;
	const watermarkImage = photon.open_image(watermarkCanvas, watermarkCtx);

	const filters: Record<string, () => void> = {
		grayscale: () => photon.grayscale(rustImage),
		offset_red: () => photon.offset(rustImage, 0, 15),
		offset_blue: () => photon.offset(rustImage, 1, 15),
		offset_green: () => photon.offset(rustImage, 2, 15),
		primary: () => photon.primary(rustImage),
		solarize: () => photon.solarize(rustImage),
		threshold: () => photon.threshold(rustImage, 200),
		sepia: () => photon.sepia(rustImage),
		decompose_min: () => photon.decompose_min(rustImage),
		decompose_max: () => photon.decompose_max(rustImage),
		grayscale_shades: () => photon.grayscale_shades(rustImage, 1),
		red_channel_grayscale: () => photon.single_channel_grayscale(rustImage, 0),
		green_channel_grayscale: () =>
			photon.single_channel_grayscale(rustImage, 1),
		blue_channel_grayscale: () => photon.single_channel_grayscale(rustImage, 2),
		hue_rotate_hsl: () => photon.hue_rotate_hsl(rustImage, 0.3),
		hue_rotate_hsv: () => photon.hue_rotate_hsv(rustImage, 0.3),
		hue_rotate_lch: () => photon.hue_rotate_lch(rustImage, 0.3),
		lighten_hsl: () => photon.lighten_hsl(rustImage, 0.1),
		lighten_hsv: () => photon.lighten_hsv(rustImage, 0.1),
		lighten_lch: () => photon.lighten_lch(rustImage, 0.1),
		darken_hsl: () => photon.darken_hsl(rustImage, 0.1),
		darken_hsv: () => photon.darken_hsv(rustImage, 0.1),
		darken_lch: () => photon.darken_lch(rustImage, 0.1),
		desaturate_hsl: () => photon.desaturate_hsl(rustImage, 0.3),
		desaturate_hsv: () => photon.desaturate_hsv(rustImage, 0.3),
		desaturate_lch: () => photon.desaturate_lch(rustImage, 0.3),
		saturate_hsl: () => photon.saturate_hsl(rustImage, 0.3),
		saturate_hsv: () => photon.saturate_hsv(rustImage, 0.3),
		saturate_lch: () => photon.saturate_lch(rustImage, 0.3),
		inc_red_channel: () => photon.alter_red_channel(rustImage, 120),
		inc_blue_channel: () => photon.alter_channel(rustImage, 2, 100),
		inc_green_channel: () => photon.alter_channel(rustImage, 1, 100),
		inc_two_channels: () => photon.alter_channel(rustImage, 1, 30),
		dec_red_channel: () => photon.alter_channel(rustImage, 0, -30),
		dec_blue_channel: () => photon.alter_channel(rustImage, 2, -30),
		dec_green_channel: () => photon.alter_channel(rustImage, 1, -30),
		swap_rg_channels: () => photon.swap_channels(rustImage, 0, 1),
		swap_rb_channels: () => photon.swap_channels(rustImage, 0, 2),
		swap_gb_channels: () => photon.swap_channels(rustImage, 1, 2),
		remove_red_channel: () => photon.remove_red_channel(rustImage, 250),
		remove_green_channel: () => photon.remove_green_channel(rustImage, 250),
		remove_blue_channel: () => photon.remove_blue_channel(rustImage, 250),
		emboss: () => photon.emboss(rustImage),
		box_blur: () => photon.box_blur(rustImage),
		sharpen: () => photon.sharpen(rustImage),
		lix: () => photon.lix(rustImage),
		neue: () => photon.neue(rustImage),
		ryo: () => photon.ryo(rustImage),
		gaussian_blur: () => photon.gaussian_blur(rustImage, 3),
		inc_brightness: () => photon.inc_brightness(rustImage, 20),
		// inc_lum: () => photon.inc_luminosity(rustImage),
		grayscale_human_corrected: () =>
			photon.grayscale_human_corrected(rustImage),
		watermark: () =>
			photon.watermark(rustImage, watermarkImage, BigInt(10), BigInt(30)),
		text: () =>
			photon.draw_text(rustImage, "welcome to WebAssembly", 10, 20, 20),
		text_border: () =>
			photon.draw_text_with_border(
				rustImage,
				"welcome to the edge",
				10,
				20,
				20,
			),
		test: () => photon.filter(rustImage, "rosetint"),
		pink_noise: () => {
			console.warn("pink_noise is unsupported in this WASM build.");
		},
		add_noise_rand: () => {
			console.warn("add_noise_rand is unsupported in this WASM build.");
		},
		apply_vibrance: () => photon.apply_vibrance(rustImage, 50.0),
		apply_dehaze: () => photon.apply_dehaze(rustImage, 60.0),
		apply_vignette: () => photon.apply_vignette(rustImage, 50.0, 30.0, 50.0),
	};

	const handler = filters[filterName];
	if (handler) {
		handler();
		photon.putImageData(canvas, ctx, rustImage);
	}

	console.timeEnd("wasm_time");
	timing.end = performance.now();
	updateBenchmarks();
	updateEffectName(target);
};

const blendImages = (event: Event, photon: ExtendedPhotonModule) => {
	if (
		!ensureContexts() ||
		!canvas2 ||
		!ctx2 ||
		!watermarkCanvas ||
		!watermarkCtx
	) {
		return;
	}
	ensureBlendCanvasSize();
	console.time("wasm_blend_time");

	ctx.drawImage(newimg, 0, 0);
	timing.start = performance.now();

	const target = event.target as HTMLElement | null;
	if (!target) {
		return;
	}
	const filterName = target.id;

	const rustImage = photon.open_image(canvas, ctx);
	const rustImage2 = photon.open_image(canvas2, ctx2);
	const watermarkImage = photon.open_image(watermarkCanvas, watermarkCtx);

	const blends: Record<string, () => void> = {
		blend: () => blendWithMode(photon, rustImage, rustImage2, "over"),
		overlay: () => blendWithMode(photon, rustImage, rustImage2, "overlay"),
		atop: () => blendWithMode(photon, rustImage, rustImage2, "atop"),
		xor: () => blendWithMode(photon, rustImage, rustImage2, "xor"),
		plus: () => blendWithMode(photon, rustImage, rustImage2, "plus"),
		multiply: () => blendWithMode(photon, rustImage, rustImage2, "multiply"),
		burn: () => blendWithMode(photon, rustImage, rustImage2, "burn"),
		difference: () =>
			blendWithMode(photon, rustImage, rustImage2, "difference"),
		soft_light: () =>
			blendWithMode(photon, rustImage, rustImage2, "soft_light"),
		hard_light: () =>
			blendWithMode(photon, rustImage, rustImage2, "hard_light"),
		dodge: () => blendWithMode(photon, rustImage, rustImage2, "dodge"),
		exclusion: () => blendWithMode(photon, rustImage, rustImage2, "exclusion"),
		lighten: () => blendWithMode(photon, rustImage, rustImage2, "lighten"),
		darken: () => blendWithMode(photon, rustImage, rustImage2, "darken"),
		watermark: () =>
			photon.watermark(rustImage, watermarkImage, BigInt(10), BigInt(30)),
		text: () =>
			photon.draw_text(rustImage, "welcome to WebAssembly", 10, 20, 20),
		text_border: () =>
			photon.draw_text_with_border(
				rustImage,
				"welcome to the edge",
				10,
				20,
				20,
			),
	};

	const handler = blends[filterName];
	if (handler) {
		handler();
		photon.putImageData(canvas, ctx, rustImage);
	}

	console.timeEnd("wasm_blend_time");
	timing.end = performance.now();
	updateBenchmarks();
	updateEffectName(target);
};

const resize = (event: Event, photon: ExtendedPhotonModule) => {
	if (!ensureContexts()) {
		return;
	}
	console.time("resize");
	const resizedContainer = document.getElementById("resized_imgs");
	if (!resizedContainer) {
		return;
	}

	ctx.drawImage(newimg, 0, 0);
	timing.start = performance.now();

	const photonImg = photon.open_image(canvas, ctx);
	const newCanvas = photon.resize_img_browser(photonImg, 200, 200, 1);
	resizedContainer.appendChild(newCanvas);
	timing.end = performance.now();
	updateBenchmarks();
	if (event.target instanceof HTMLElement) {
		updateEffectName(event.target);
	}
	console.timeEnd("resize");
};

const updateEffectName = (elem: HTMLElement) => {
	const effectName = elem.innerHTML;
	const effectElem = document.getElementById("effect_name");
	if (effectElem) {
		effectElem.innerHTML = effectName;
	}
};

const changeImage = (event: Event) => {
	const target = event.target as HTMLElement;
	const imgName = target.id;
	const imgNamesToImages: Record<string, string> = {
		underground: Underground,
		blue_metro: BlueMetro,
		nine_yards: NineYards,
		fruit: MainImage,
	};

	const src = imgNamesToImages[imgName];
	if (!src) {
		return;
	}

	newimg.src = src;
	newimg.onload = () => {
		canvas.width = newimg.width;
		canvas.height = newimg.height;
		ctx.drawImage(newimg, 0, 0);
		ensureBlendCanvasSize();
	};
};

const changeImageFromNav = () => {
	newimg.src = Underground;
	newimg.onload = () => {
		canvas.width = newimg.width;
		canvas.height = newimg.height;
		ctx.drawImage(newimg, 0, 0);
		ensureBlendCanvasSize();
	};
};

const setUpCanvas = () => {
	const container = document.getElementById("image_container");
	container?.appendChild(newimg);

	canvas = document.getElementById("canvas") as HTMLCanvasElement;
	canvas.width = newimg.width;
	canvas.height = newimg.height;
	const context = canvas.getContext("2d");
	if (!context) {
		throw new Error("Unable to get canvas context.");
	}
	ctx = context;
	ctx.drawImage(newimg, 0, 0);
	ensureBlendCanvasSize();
};

const setUpCanvas2 = () => {
	const container = document.getElementById("image_container");
	container?.appendChild(img2);
	canvas2 = document.createElement("canvas");
	canvas2.width = img2.width;
	canvas2.height = img2.height;
	const context = canvas2.getContext("2d");
	if (!context) {
		throw new Error("Unable to get secondary canvas context.");
	}
	ctx2 = context;
	ctx2.drawImage(img2, 0, 0);
};

const setUpWatermark = () => {
	const container = document.getElementById("image_container");
	container?.appendChild(watermarkImg);
	watermarkCanvas = document.createElement("canvas");
	watermarkCanvas.width = watermarkImg.width;
	watermarkCanvas.height = watermarkImg.height;
	const context = watermarkCanvas.getContext("2d");
	if (!context) {
		throw new Error("Unable to get watermark canvas context.");
	}
	watermarkCtx = context;
	watermarkCtx.drawImage(watermarkImg, 0, 0);
};

const updateBenchmarks = () => {
	const timeTaken = timing.end - timing.start;
	const timeElem = document.getElementById("time");
	if (timeElem) {
		timeElem.innerHTML = `Time: ${timeTaken.toFixed(2)}ms`;
	}
};

const editImage = (
	activeCanvas: HTMLCanvasElement,
	context: CanvasRenderingContext2D,
) => {
	const imgData = context.getImageData(
		0,
		0,
		activeCanvas.width,
		activeCanvas.height,
	);
	for (let i = 0; i < imgData.data.length; i += 4) {
		imgData.data[i] += 30;
	}
	context.putImageData(imgData, 0, 0);
};

if (document.readyState === "loading") {
	document.addEventListener("DOMContentLoaded", () => {
		void bootstrap();
	});
} else {
	void bootstrap();
}
