/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".bootstrap.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"./pkg/index_bg.wasm": function() {
/******/ 			return {
/******/ 				"./index_bg.js": {
/******/ 					"__wbg_playse_0324422c63ad3a04": function(p0i32,p1i32,p2i32,p3f32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_playse_0324422c63ad3a04"](p0i32,p1i32,p2i32,p3f32);
/******/ 					},
/******/ 					"__wbg_playloop_9da26547aca439f3": function(p0i32,p1i32,p2i32,p3f32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_playloop_9da26547aca439f3"](p0i32,p1i32,p2i32,p3f32);
/******/ 					},
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbg_error_4bb6c2a97407129a": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_error_4bb6c2a97407129a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_new_59cb74e423758ede": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_new_59cb74e423758ede"]();
/******/ 					},
/******/ 					"__wbg_stack_558ba5917b466edd": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_stack_558ba5917b466edd"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_cb_drop": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_cb_drop"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Window_49f532f06a9786ee": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_instanceof_Window_49f532f06a9786ee"](p0i32);
/******/ 					},
/******/ 					"__wbg_document_c0366b39e4f4c89a": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_document_c0366b39e4f4c89a"](p0i32);
/******/ 					},
/******/ 					"__wbg_fetch_f532e04b8fe49aa0": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_fetch_f532e04b8fe49aa0"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getElementById_15aef17a620252b4": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getElementById_15aef17a620252b4"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Response_f52c65c389890639": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_instanceof_Response_f52c65c389890639"](p0i32);
/******/ 					},
/******/ 					"__wbg_text_afdc7a1dc7edda52": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_text_afdc7a1dc7edda52"](p0i32);
/******/ 					},
/******/ 					"__wbg_headers_5f70b64960218e2f": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_headers_5f70b64960218e2f"](p0i32);
/******/ 					},
/******/ 					"__wbg_newwithstrandinit_11debb554792e043": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_newwithstrandinit_11debb554792e043"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlCanvasElement_7bd3ee7838f11fc3": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_instanceof_HtmlCanvasElement_7bd3ee7838f11fc3"](p0i32);
/******/ 					},
/******/ 					"__wbg_width_0efa4604d41c58c5": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_width_0efa4604d41c58c5"](p0i32);
/******/ 					},
/******/ 					"__wbg_height_aa24e3fef658c4a8": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_height_aa24e3fef658c4a8"](p0i32);
/******/ 					},
/******/ 					"__wbg_getContext_3db9399e6dc524ff": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getContext_3db9399e6dc524ff"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_error_e325755affc8634b": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_error_e325755affc8634b"](p0i32);
/******/ 					},
/******/ 					"__wbg_setonload_6fb79f39cb59de67": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_setonload_6fb79f39cb59de67"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setonerror_bf29bf699c956162": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_setonerror_bf29bf699c956162"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_instanceof_CanvasRenderingContext2d_1d38418d1d6c8b34": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_instanceof_CanvasRenderingContext2d_1d38418d1d6c8b34"](p0i32);
/******/ 					},
/******/ 					"__wbg_setglobalAlpha_b4134dbd884b4d37": function(p0i32,p1f64) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_setglobalAlpha_b4134dbd884b4d37"](p0i32,p1f64);
/******/ 					},
/******/ 					"__wbg_setfillStyle_bbe97cb93eb1b55c": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_setfillStyle_bbe97cb93eb1b55c"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setfont_8a2c2ff87f58b6bc": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_setfont_8a2c2ff87f58b6bc"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_drawImage_e6d293e6dbfa15b6": function(p0i32,p1i32,p2f64,p3f64,p4f64,p5f64) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_drawImage_e6d293e6dbfa15b6"](p0i32,p1i32,p2f64,p3f64,p4f64,p5f64);
/******/ 					},
/******/ 					"__wbg_drawImage_767d2c71080c29e4": function(p0i32,p1i32,p2f64,p3f64,p4f64,p5f64,p6f64,p7f64,p8f64,p9f64) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_drawImage_767d2c71080c29e4"](p0i32,p1i32,p2f64,p3f64,p4f64,p5f64,p6f64,p7f64,p8f64,p9f64);
/******/ 					},
/******/ 					"__wbg_fillRect_c79d1b386c04efed": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_fillRect_c79d1b386c04efed"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__wbg_restore_c7f054400aabc851": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_restore_c7f054400aabc851"](p0i32);
/******/ 					},
/******/ 					"__wbg_save_cbb4c6bab9227d88": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_save_cbb4c6bab9227d88"](p0i32);
/******/ 					},
/******/ 					"__wbg_fillText_4169c5cb3696ff0a": function(p0i32,p1i32,p2i32,p3f64,p4f64,p5f64) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_fillText_4169c5cb3696ff0a"](p0i32,p1i32,p2i32,p3f64,p4f64,p5f64);
/******/ 					},
/******/ 					"__wbg_rotate_a0fdfa1b3b7b875e": function(p0i32,p1f64) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_rotate_a0fdfa1b3b7b875e"](p0i32,p1f64);
/******/ 					},
/******/ 					"__wbg_translate_b4a840fe0044bc44": function(p0i32,p1f64,p2f64) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_translate_b4a840fe0044bc44"](p0i32,p1f64,p2f64);
/******/ 					},
/******/ 					"__wbg_setsrc_ca894d724570195d": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_setsrc_ca894d724570195d"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_new_1f2ded01c889cd38": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_new_1f2ded01c889cd38"]();
/******/ 					},
/******/ 					"__wbg_set_76c6c83392bce553": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_set_76c6c83392bce553"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_7c6bd521992b4022": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_newnoargs_7c6bd521992b4022"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_951bd0c6d815d6f1": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_call_951bd0c6d815d6f1"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_new_ba07d0daa0e4677e": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_new_ba07d0daa0e4677e"]();
/******/ 					},
/******/ 					"__wbg_resolve_6e61e640925a0db9": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_resolve_6e61e640925a0db9"](p0i32);
/******/ 					},
/******/ 					"__wbg_then_dd3785597974798a": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_then_dd3785597974798a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_then_0f957e0f4c3e537a": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_then_0f957e0f4c3e537a"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_globalThis_513fb247e8e4e6d2": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_globalThis_513fb247e8e4e6d2"]();
/******/ 					},
/******/ 					"__wbg_self_6baf3a3aa7b63415": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_self_6baf3a3aa7b63415"]();
/******/ 					},
/******/ 					"__wbg_window_63fc4027b66c265b": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_window_63fc4027b66c265b"]();
/******/ 					},
/******/ 					"__wbg_global_b87245cd886d7113": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_global_b87245cd886d7113"]();
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbg_set_9bdd413385146137": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_set_9bdd413385146137"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_self_1c83eb4471d9eb9b": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_self_1c83eb4471d9eb9b"]();
/******/ 					},
/******/ 					"__wbg_msCrypto_679be765111ba775": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_msCrypto_679be765111ba775"](p0i32);
/******/ 					},
/******/ 					"__wbg_crypto_c12f14e810edcaa2": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_crypto_c12f14e810edcaa2"](p0i32);
/******/ 					},
/******/ 					"__wbg_getRandomValues_05a60bf171bfc2be": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getRandomValues_05a60bf171bfc2be"](p0i32);
/******/ 					},
/******/ 					"__wbg_getRandomValues_3ac1b33c90b52596": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getRandomValues_3ac1b33c90b52596"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_randomFillSync_6f956029658662ec": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_randomFillSync_6f956029658662ec"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_static_accessor_MODULE_abf5ae284bffdf45": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_static_accessor_MODULE_abf5ae284bffdf45"]();
/******/ 					},
/******/ 					"__wbg_require_5b2b5b594d809d9f": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_require_5b2b5b594d809d9f"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_number_get": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_number_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_string_get": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_string_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper1611": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_closure_wrapper1611"](p0i32,p1i32,p2i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["./pkg/index_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"./pkg/index_bg.wasm":"ab882519f2418b700921"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./bootstrap.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./bootstrap.js":
/*!**********************!*\
  !*** ./bootstrap.js ***!
  \**********************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("// A dependency graph that contains any wasm must all be imported\n// asynchronously. This `bootstrap.js` file does the single async import, so\n// that no one else needs to worry about it again.\n__webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ./index.js */ \"./index.js\"))\n  .catch(e => console.error(\"Error importing `index.js`:\", e));\n\n\n//# sourceURL=webpack:///./bootstrap.js?");

/***/ })

/******/ });