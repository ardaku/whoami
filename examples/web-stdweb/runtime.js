var _cala_heap = new Array();
var _cala_garbage = new Array();
function _cala_js_malloc(o) {
    if(_cala_garbage.length != 0) {
        let idx = _cala_garbage.pop();
        _cala_heap[idx] = o;
        return idx;
    } else {
        let idx = _cala_heap.length;
        _cala_heap.push(o);
        return idx;
    }
}
function _cala_js_text(p,l) {
    var buf = new Uint16Array(Module.instance.exports.memory.buffer,p,l);
    var str = "";
    for(var i = 0; i < l; i++) {
        str += String.fromCharCode(buf[i]);
    }
    return _cala_js_malloc(str);
}
function _cala_js_copy(dst,src,len) {
    for(var i = 0; i < len; i++) { dst[i] = src[i]; }
}
function _cala_js_write_bytes(j,p,l) {
    var d = new Uint8Array(Module.instance.exports.memory.buffer,p,l);
    _cala_js_copy(_cala_heap[j],d,l);
}
function _cala_js_write_ints(j,p,l) {
    var d = new Int32Array(Module.instance.exports.memory.buffer,p,l);
    _cala_js_copy(_cala_heap[j],d,l);
}
function _cala_js_write_floats(j,p,l) {
    var d = new Float32Array(Module.instance.exports.memory.buffer,p,l);
    _cala_js_copy(_cala_heap[j],d,l);
}
function _cala_js_write_doubles(j,p,l) {
    var d = new Float64Array(Module.instance.exports.memory.buffer,p,l);
    _cala_js_copy(_cala_heap[j],d,l);
}
function _cala_js_function(i) {
    return _cala_js_malloc(Function(_cala_heap[i])());
}
function _cala_js_call(f, a, b) {
    let o = _cala_heap[f](_cala_heap[a], _cala_heap[b]);
    if(o == undefined) {
        return -1;
    } else {
        return _cala_js_malloc(o);
    }
}
function _cala_js_free(i) { return _cala_garbage.push(i); }
function _cala_js_read_text(j,p,l) {
    var buf = new Uint16Array(Module.instance.exports.memory.buffer,p,l);
    let get = _cala_heap[j];
    for(var i = 0; i < l; i++) {
        buf[i] = get.charCodeAt(i);
    }
    return get.length;
}
function _cala_js_read_copy(buf,get,len) {
    for(var i = 0; i < len; i++) {
        buf[i] = get[i];
    }
    return get.length;
}
function _cala_js_read_bytes(j,p,l) {
    var buf = new Uint8Array(Module.instance.exports.memory.buffer,p,l);
    return _cala_js_read_copy(buf,_cala_heap[j],l);
}
function _cala_js_read_ints(j, p, l) {
    var buf = new Int32Array(Module.instance.exports.memory.buffer,p,l);
    return _cala_js_read_copy(buf,_cala_heap[j],l);
}
function _cala_js_read_floats(j, p, l) {
    var buf = new Float32Array(Module.instance.exports.memory.buffer,p,l);
    return _cala_js_read_copy(buf,_cala_heap[j],l);
}
function _cala_js_read_doubles(j, p, l) {
    var buf = new Float64Array(Module.instance.exports.memory.buffer,p,l);
    return _cala_js_read_copy(buf,_cala_heap[j],l);
}
function _cala_js_waker(j) {
    _cala_heap[j].then((o) => {
        if(o == undefined || o == null) {
            Module.instance.exports.wake(j, -1);
        } else {
            Module.instance.exports.wake(j, _cala_js_malloc(o))
        }
    });
}
function _cala_js_store_int(o) { return _cala_js_malloc(o); }
function _cala_js_load_int(o) { return _cala_heap[o]; }
function _cala_js_store_float(o) { return _cala_js_malloc(o); }
function _cala_js_load_float(o) { return _cala_heap[o]; }
function _cala_js_store_double(o) { return _cala_js_malloc(o); }
function _cala_js_load_double(o) { return _cala_heap[o]; }
