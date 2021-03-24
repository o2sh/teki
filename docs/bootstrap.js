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
/******/ 					"__wbg_instanceof_Window_5993230e7331f098": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_instanceof_Window_5993230e7331f098"](p0i32);
/******/ 					},
/******/ 					"__wbg_document_85584f745133c6ad": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_document_85584f745133c6ad"](p0i32);
/******/ 					},
/******/ 					"__wbg_fetch_d2665d527f7f4a02": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_fetch_d2665d527f7f4a02"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getElementById_85c96642ffb33978": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getElementById_85c96642ffb33978"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Response_5a8048ec57ecf2d5": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_instanceof_Response_5a8048ec57ecf2d5"](p0i32);
/******/ 					},
/******/ 					"__wbg_text_4b57f36d42790087": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_text_4b57f36d42790087"](p0i32);
/******/ 					},
/******/ 					"__wbg_headers_e44f681c4a625201": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_headers_e44f681c4a625201"](p0i32);
/******/ 					},
/******/ 					"__wbg_newwithstrandinit_814c15649f6924a8": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_newwithstrandinit_814c15649f6924a8"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlCanvasElement_46dcfe68d7a9fa74": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_instanceof_HtmlCanvasElement_46dcfe68d7a9fa74"](p0i32);
/******/ 					},
/******/ 					"__wbg_width_203c80aa13d6e5d4": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_width_203c80aa13d6e5d4"](p0i32);
/******/ 					},
/******/ 					"__wbg_height_ae75a4b0f6bd1f84": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_height_ae75a4b0f6bd1f84"](p0i32);
/******/ 					},
/******/ 					"__wbg_getContext_cbecd1fc57539f80": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getContext_cbecd1fc57539f80"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_error_e25c602bf2cc97d2": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_error_e25c602bf2cc97d2"](p0i32);
/******/ 					},
/******/ 					"__wbg_setonload_2c70680e2c604b4e": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_setonload_2c70680e2c604b4e"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setonerror_3cb912accad24184": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_setonerror_3cb912accad24184"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_instanceof_CanvasRenderingContext2d_2fc2819b8ff4979a": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_instanceof_CanvasRenderingContext2d_2fc2819b8ff4979a"](p0i32);
/******/ 					},
/******/ 					"__wbg_setglobalAlpha_87b69cc70d624eab": function(p0i32,p1f64) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_setglobalAlpha_87b69cc70d624eab"](p0i32,p1f64);
/******/ 					},
/******/ 					"__wbg_setfillStyle_1b018f07574a0711": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_setfillStyle_1b018f07574a0711"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setfont_12950834cb972674": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_setfont_12950834cb972674"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_drawImage_28476c1cab2c9c45": function(p0i32,p1i32,p2f64,p3f64,p4f64,p5f64) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_drawImage_28476c1cab2c9c45"](p0i32,p1i32,p2f64,p3f64,p4f64,p5f64);
/******/ 					},
/******/ 					"__wbg_drawImage_551ff1a7269d3884": function(p0i32,p1i32,p2f64,p3f64,p4f64,p5f64,p6f64,p7f64,p8f64,p9f64) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_drawImage_551ff1a7269d3884"](p0i32,p1i32,p2f64,p3f64,p4f64,p5f64,p6f64,p7f64,p8f64,p9f64);
/******/ 					},
/******/ 					"__wbg_fillRect_6ed2d624fc222ab1": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_fillRect_6ed2d624fc222ab1"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__wbg_restore_9a1e268169125fba": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_restore_9a1e268169125fba"](p0i32);
/******/ 					},
/******/ 					"__wbg_save_0d2d5b76db8ce8c1": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_save_0d2d5b76db8ce8c1"](p0i32);
/******/ 					},
/******/ 					"__wbg_fillText_406b7ce3001419e9": function(p0i32,p1i32,p2i32,p3f64,p4f64,p5f64) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_fillText_406b7ce3001419e9"](p0i32,p1i32,p2i32,p3f64,p4f64,p5f64);
/******/ 					},
/******/ 					"__wbg_rotate_e86541abb8b30d34": function(p0i32,p1f64) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_rotate_e86541abb8b30d34"](p0i32,p1f64);
/******/ 					},
/******/ 					"__wbg_translate_6f0a02e9e410717d": function(p0i32,p1f64,p2f64) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_translate_6f0a02e9e410717d"](p0i32,p1f64,p2f64);
/******/ 					},
/******/ 					"__wbg_setsrc_9cc83ec1922ff76a": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_setsrc_9cc83ec1922ff76a"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_new_124fd8c3abad616a": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_new_124fd8c3abad616a"]();
/******/ 					},
/******/ 					"__wbg_set_53cbfc6fa4c07f2b": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_set_53cbfc6fa4c07f2b"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_self_86b4b13392c7af56": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_self_86b4b13392c7af56"]();
/******/ 					},
/******/ 					"__wbg_msCrypto_9ad6677321a08dd8": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_msCrypto_9ad6677321a08dd8"](p0i32);
/******/ 					},
/******/ 					"__wbg_crypto_b8c92eaac23d0d80": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_crypto_b8c92eaac23d0d80"](p0i32);
/******/ 					},
/******/ 					"__wbg_getRandomValues_dd27e6b0652b3236": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getRandomValues_dd27e6b0652b3236"](p0i32);
/******/ 					},
/******/ 					"__wbg_getRandomValues_e57c9b75ddead065": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getRandomValues_e57c9b75ddead065"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_randomFillSync_d2ba53160aec6aba": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_randomFillSync_d2ba53160aec6aba"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_static_accessor_MODULE_452b4680e8614c81": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_static_accessor_MODULE_452b4680e8614c81"]();
/******/ 					},
/******/ 					"__wbg_require_f5521a5b85ad2542": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_require_f5521a5b85ad2542"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_2349ba6aefe72376": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_newnoargs_2349ba6aefe72376"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_e5847d15cc228e4f": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_call_e5847d15cc228e4f"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_new_59986c8731bebaa1": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_new_59986c8731bebaa1"]();
/******/ 					},
/******/ 					"__wbg_resolve_e0690143406c88cb": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_resolve_e0690143406c88cb"](p0i32);
/******/ 					},
/******/ 					"__wbg_then_9caf23122e4fd5d3": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_then_9caf23122e4fd5d3"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_then_16663faf60ffbe95": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_then_16663faf60ffbe95"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_globalThis_1d843c4ad7b6a1f5": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_globalThis_1d843c4ad7b6a1f5"]();
/******/ 					},
/******/ 					"__wbg_self_35a0fda3eb965abe": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_self_35a0fda3eb965abe"]();
/******/ 					},
/******/ 					"__wbg_window_88a6f88dd3a474f1": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_window_88a6f88dd3a474f1"]();
/******/ 					},
/******/ 					"__wbg_global_294ce70448e8fbbf": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_global_294ce70448e8fbbf"]();
/******/ 					},
/******/ 					"__wbg_new_4e8d18dbf9cd5240": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_new_4e8d18dbf9cd5240"](p0i32);
/******/ 					},
/******/ 					"__wbg_newwithlength_19241666d161c55f": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_newwithlength_19241666d161c55f"](p0i32);
/******/ 					},
/******/ 					"__wbg_subarray_b07d46fd5261d77f": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_subarray_b07d46fd5261d77f"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_length_3a5138f465b971ad": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_length_3a5138f465b971ad"](p0i32);
/******/ 					},
/******/ 					"__wbg_set_4769de301eb521d7": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_set_4769de301eb521d7"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_buffer_0be9fb426f2dd82b": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_buffer_0be9fb426f2dd82b"](p0i32);
/******/ 					},
/******/ 					"__wbg_set_7e15d36563072b19": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_set_7e15d36563072b19"](p0i32,p1i32,p2i32);
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
/******/ 					"__wbindgen_memory": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_memory"]();
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper1760": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_closure_wrapper1760"](p0i32,p1i32,p2i32);
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
/******/ 				var req = fetch(__webpack_require__.p + "" + {"./pkg/index_bg.wasm":"f27a6b5e1dad8b4910de"}[wasmModuleId] + ".module.wasm");
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