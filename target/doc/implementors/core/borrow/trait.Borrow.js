(function() {var implementors = {};
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + <a class=\"trait\" href=\"crossbeam_epoch/trait.Pointable.html\" title=\"trait crossbeam_epoch::Pointable\">Pointable</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;T&gt; for <a class=\"struct\" href=\"crossbeam_epoch/struct.Owned.html\" title=\"struct crossbeam_epoch::Owned\">Owned</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_epoch::atomic::Owned"]}];
implementors["generic_array"] = [{"text":"impl&lt;T, N&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/core/primitive.slice.html\">[</a>T<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/core/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"generic_array/struct.GenericArray.html\" title=\"struct generic_array::GenericArray\">GenericArray</a>&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"generic_array/trait.ArrayLength.html\" title=\"trait generic_array::ArrayLength\">ArrayLength</a>&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":["generic_array::GenericArray"]}];
implementors["sled"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"sled/struct.IVec.html\" title=\"struct sled::IVec\">IVec</a>","synthetic":false,"types":["sled::ivec::IVec"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/std/primitive.slice.html\">]</a>&gt; for &amp;<a class=\"struct\" href=\"sled/struct.IVec.html\" title=\"struct sled::IVec\">IVec</a>","synthetic":false,"types":["sled::ivec::IVec"]}];
implementors["smallvec"] = [{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/core/primitive.slice.html\">[</a>&lt;A as <a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt;::<a class=\"type\" href=\"smallvec/trait.Array.html#associatedtype.Item\" title=\"type smallvec::Array::Item\">Item</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.58.1/core/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"smallvec/struct.SmallVec.html\" title=\"struct smallvec::SmallVec\">SmallVec</a>&lt;A&gt;","synthetic":false,"types":["smallvec::SmallVec"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()