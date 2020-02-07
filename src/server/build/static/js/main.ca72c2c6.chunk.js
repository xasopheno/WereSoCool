(window.webpackJsonp=window.webpackJsonp||[]).push([[0],{54:function(e,t,n){e.exports=n(94)},84:function(e,t,n){"use strict";(function(e){var t=n(31),a=n.n(t);a.a.define("ace/theme/wsc",["require","exports","module","ace/lib/dom"],function(e,t,n){t.isDark=!0,t.cssClass="ace-wsc-theme",t.cssText=".ace-wsc-theme .ace_gutter {background: #1a0005;}.ace-wsc-theme {background: #1a0005;color: #929292  font-color: red}.ace-wsc-theme .ace_print-margin {width: 1px;background: #1a1a1a}.ace-wsc-theme { background-color: #111102;color: #DEDEDE}.ace-wsc-theme .ace_cursor {color: #9F9F9F}.ace-wsc-theme .ace_marker-layer .ace_selection {background: #424242}.ace-wsc-theme.ace_multiselect .ace_selection.ace_start {box-shadow: 0 0 3px 0px black;}.ace-wsc-theme .ace_marker-layer .ace_step {background: rgb(0, 0, 0)}.ace-wsc-theme .ace_marker-layer .ace_bracket {background: salmon;}.ace-wsc-theme .ace_marker-layer .ace_bracket-start {background: orange;}.ace-wsc-theme .ace_marker-layer .ace_bracket-unmatched {margin: -1px 0 0 -1px;border: 1px solid #900}.ace-wsc-theme .ace_marker-layer .ace_active-line {background: #2A2A2A}.ace-wsc-theme .ace_gutter-active-line {background-color: #2A112A}.ace-wsc-theme .ace_invisible {color: #343434}.ace-wsc-theme .ace_operation {color: #ff6168}.ace-wsc-theme .ace_danny {color: deeppink}.ace-wsc-theme .ace_group_operation {color: steelblue}.ace-wsc-theme .ace_group_operation_other {color: #FFD866}.ace-wsc-theme .ace_o_shortcut {color: #E78C45}.ace-wsc-theme .ace_pipe {color: #D54E53}.ace-wsc-theme .ace_bracket {color: #7AA6DA}.ace-wsc-theme .ace_curly {color: tomato}.ace-wsc-theme .ace_repeat {color: #A9DC76}.ace-wsc-theme .ace_comment {color: grey}.ace-wsc-theme .ace_number {color: cornsilk}.ace-wsc-theme .ace_slash {color: tan}.ace-wsc-theme .ace_zero {color: wheat}.ace-wsc-theme .ace_zero {color: wheat}.ace-wsc-theme .ace_letter {color: #FFCDFF}.ace-wsc-theme .ace_string {color: #B9CA4A}.error {  background: lightsalmon;  opacity: 0.2;  position:absolute;  z-index:20;}",e("../lib/dom").importCssString(t.cssText,t.cssClass)}),a.a.require(["ace/theme/wsc"],function(t){"object"==typeof exports&&e&&(e.exports=t)})}).call(this,n(85)(e))},93:function(e,t,n){},94:function(e,t,n){"use strict";n.r(t);var a=n(0),r=n.n(a),o=n(22),c=n.n(o),i=n(8),s=n(13),u=n(15),l=n(14),d=n(16),h=n(30),f=n(18),m=n(1),p=n.n(m),v=n(6),b=n(5),g=n(48),w=n.n(g),y=n(21),O=n.n(y),j=(n(84),n(2)),k=n(3);function x(){var e=Object(j.a)(["\n  display: flex;\n  justify-content: space-between;\n"]);return x=function(){return e},e}function E(){var e=Object(j.a)(["\n  font-size: 1em;\n  background-color: #454343;\n  margin-right: 2em;\n  color: #edd;\n  &:visited {\n    outline: none;\n  }\n  &:active {\n    outline: none;\n  }\n  &:focus {\n    outline: none;\n  }\n"]);return E=function(){return e},e}function _(){var e=Object(j.a)(["\n  display: flex;\n  flex-direction: row;\n  justify-content: flex-start;\n  margin-left: 10%;\n"]);return _=function(){return e},e}function C(){var e=Object(j.a)(["\n  vertical-align: middle;\n  margin-bottom: 0px;\n"]);return C=function(){return e},e}function S(){var e=Object(j.a)(["\n  font-family: 'Courier New', Courier, monospace;\n  text-align: bottom;\n  color: #cbb;\n  font-size: 1em;\n  padding-right: 0.2em;\n"]);return S=function(){return e},e}function A(){var e=Object(j.a)(["\n  display: flex;\n  flex-direction: row;\n  justify-content: flex-end;\n  margin-right: 10%;\n"]);return A=function(){return e},e}function z(){var e=Object(j.a)(["\n  background-color: #454343;\n  height: 100vh;\n"]);return z=function(){return e},e}function P(){var e=Object(j.a)(["\n  font-family: 'Courier New', Courier, monospace;\n  //text-align: center;\n  color: ",";\n  font-size: 1em;\n  margin-top: 0;\n  margin-bottom: 0;\n  padding-top: 0;\n  padding-bottom: 0;\n  color: gold;\n"]);return P=function(){return e},e}function R(){var e=Object(j.a)(["\n  font-family: 'Courier New', Courier, monospace;\n  text-align: center;\n  color: #cbb;\n  font-size: 1em;\n"]);return R=function(){return e},e}function D(){var e=Object(j.a)(["\n  font-family: 'Courier New', Courier, monospace;\n  text-align: center;\n  padding-top: 10px;\n  color: #edd;\n  font-size: 1.5em;\n"]);return D=function(){return e},e}var T,L=k.a.h1(D()),N=k.a.p(R()),U=k.a.p(P(),function(e){return e.color}),B=k.a.div(z()),F=k.a.div(A()),M=k.a.label(S()),W=k.a.input(C()),V=k.a.div(_()),I=k.a.button(E()),H=k.a.div(x()),q=(n(87),function(e){function t(){var e;return Object(i.a)(this,t),(e=Object(u.a)(this,Object(l.a)(t).call(this))).$rules={start:[{token:"comment",regex:"--.*$"},{token:"number",regex:"[1-9]"},{token:"zero",regex:"[0]"},{token:"danny",regex:"f:|l:|g:|p:"},{token:"slash",regex:"/"},{token:"keyword",regex:"#"},{token:"curly",regex:"{|}"},{token:"bracket",regex:"\\[|\\]"},{token:"paren",regex:"\\(|\\)"},{token:"pipe",regex:"\\|"},{token:"keyword",regex:">"},{token:"curly",regex:"="},{token:"dot",regex:"\\."},{token:"group_operation_other",regex:"FitLength|ModulateBy|Reverse"},{token:"repeat",regex:"Repeat"},{token:"group_operation",regex:"Sequence|Overlay|Seq"},{token:"o_shortcut",regex:"O"},{token:"operation",regex:"AsIs|Tm|Ta|PanA|PanM|Gain|Length"},{token:"letter",regex:"[a-z]"}]},e}return Object(d.a)(t,e),t}(window.ace.acequire("ace/mode/text_highlight_rules").TextHighlightRules)),G=function(e){function t(){var e;return Object(i.a)(this,t),(e=Object(u.a)(this,Object(l.a)(t).call(this))).HighlightRules=q,e.lineCommentStart="--",e.getNextLineIndent=function(e,t,n){var a=this.$getIndent(t),r=this.getTokenizer().getLineTokens(t,e).tokens;if(r.length&&"comment"===r[r.length-1].type)return a;"start"===e&&(t.match(/^.*[{([]\s*$/)&&(a+=n));return a},e}return Object(d.a)(t,e),t}(window.ace.acequire("ace/mode/text").Mode),J="{ f: 285, l: 1, g: 1, p: 0 }\n\novertones = {\n  O[\n    (1/1, 2, 1, 1),\n    (1/1, 0, 1, -1),\n  ]\n}\n\nthing1 = {\n  Seq [\n    Tm 1, Tm 9/8, Tm 5/4\n  ]\n}\n\nmain = {\n  overtones\n  | thing1\n}\n",$=function(e,t,n,a){var r=new Float32Array(t.l_buffer),o=new Float32Array(t.r_buffer),c=e.createBufferSource(),i=e.createBuffer(2,r.length,e.sampleRate);i.copyToChannel(r,0),i.copyToChannel(o,1);var s=new GainNode(e);s.connect(e.destination),c.buffer=i,c.connect(s),c.start(),n(c),a(s)},K=function(e,t,n){t&&(n.gain.exponentialRampToValueAtTime(1e-4,e.currentTime+.05),t.stop(e.currentTime+.05))},X=function(e,t,n,a){a([Y(e.line,e.column,t)]),n.editor.gotoLine(e.line,e.column)},Y=function(e,t,n){return{startRow:e-=1,startCol:t,endRow:n,endCol:0,type:"text",className:"error"}},Z=(n(88),n(89),n(90),"http://0.0.0.0:8000/api/render"),Q=new G,ee=new AudioContext;!function(e){e[e.Cool=0]="Cool",e[e.Rendering=1]="Rendering"}(T||(T={}));var te=function(){var e=Object(a.useState)(T.Cool),t=Object(b.a)(e,2),n=t[0],o=t[1],c=Object(a.useState)(!1),i=Object(b.a)(c,2),s=i[0],u=i[1],l=Object(a.useState)(!0),d=Object(b.a)(l,2),h=d[0],f=d[1],m=Object(a.useState)(),g=Object(b.a)(m,2),y=g[0],j=g[1],k=Object(a.useState)(J),x=Object(b.a)(k,2),E=x[0],_=x[1],C=Object(a.useState)(!1),S=Object(b.a)(C,2),A=S[0],z=S[1],P=Object(a.useState)(0),R=Object(b.a)(P,2),D=R[0],q=R[1],G=Object(a.useState)(null),Y=Object(b.a)(G,2),te=Y[0],ne=Y[1],ae=Object(a.useState)(null),re=Object(b.a)(ae,2),oe=re[0],ce=re[1],ie=Object(a.useState)(new GainNode(ee)),se=Object(b.a)(ie,2),ue=se[0],le=se[1],de=Object(a.useState)(new GainNode(ee)),he=Object(b.a)(de,2),fe=he[0],me=he[1],pe=Object(a.useState)([]),ve=Object(b.a)(pe,2),be=ve[0],ge=ve[1],we=[ue,fe],ye=[le,me],Oe=[te,oe],je=[ne,ce];return Object(a.useEffect)(function(){!function(){var e=Object(v.a)(p.a.mark(function e(){var t,n,a,r,c;return p.a.wrap(function(e){for(;;)switch(e.prev=e.next){case 0:if(!A){e.next=34;break}return ge([]),q((D+1)%2),t=je[D],n=ye[D],K(ee,Oe[(D+1)%2],we[(D+1)%2]),e.prev=8,o(T.Rendering),e.next=12,O.a.post(Z,{language:E});case 12:if(a=e.sent,!y){e.next=27;break}e.t0=a.data.response_type,e.next="RenderSuccess"===e.t0?17:"RenderError"===e.t0?20:24;break;case 17:return localStorage.setItem("language",E),$(ee,a.data.buffers,t,n),e.abrupt("break",27);case 20:return r=a.data.error,c=E.split("\n").length,X(r,c,y,ge),e.abrupt("break",27);case 24:return console.log("Not sure how we got here..."),console.log(a),e.abrupt("break",27);case 27:e.next=32;break;case 29:e.prev=29,e.t1=e.catch(8),console.log(e.t1);case 32:o(T.Cool),z(!1);case 34:case"end":return e.stop()}},e,null,[[8,29]])}));return function(){return e.apply(this,arguments)}}()()},[A]),Object(a.useEffect)(function(){y&&(y.editor.getSession().setMode(Q),y.editor.setTheme("ace/theme/wsc"))},[y]),Object(a.useEffect)(function(){!function(){var e=localStorage.getItem("language");e&&_(e)}()},[]),Object(a.useEffect)(function(){s&&u(!1)},[s]),r.a.createElement(B,null,r.a.createElement(L,null,"WereSoCool"),r.a.createElement(N,null,"Make cool sounds. Impress your friends/pets/plants."),r.a.createElement(H,null,r.a.createElement(V,null,r.a.createElement(I,{onClick:function(){return z(!0)}},"Render"),r.a.createElement(I,{onClick:function(){K(ee,Oe[(D+1)%2],we[(D+1)%2])}},"Stop"),r.a.createElement(function(){return n===T.Rendering?r.a.createElement(U,null,"Rendering"):r.a.createElement("div",null)},null)),r.a.createElement(F,null,r.a.createElement(I,{onClick:function(){return _(J)}},"Reset"),r.a.createElement(M,null,h?"Vim!":"Vim?"),r.a.createElement(W,{name:"Vim",type:"checkbox",checked:!!h,onChange:function(){return f(!h)}}))),r.a.createElement(w.a,{focus:!0,ref:function(e){j(e)},placeholder:"WereSoCool",mode:"elixir",theme:"github",name:"aceEditor",keyboardHandler:h?"vim":"",value:E,onChange:function(e){return _(e)},markers:be,fontSize:20,showPrintMargin:!0,showGutter:!0,highlightActiveLine:!0,setOptions:{showLineNumbers:!0,tabSize:2},commands:[{name:"submit",bindKey:{win:"Shift-Enter",mac:"Shift-Enter"},exec:function(){z(!0)}},{name:"save",bindKey:{win:"Command-s",mac:"Command-s"},exec:function(){u(!0)}}],style:{height:"80vh",width:"80vw",marginLeft:"10vw"}}))},ne=n(34),ae=n(24),re=n(11),oe=(Object({NODE_ENV:"production",PUBLIC_URL:""}).PORT,function(){function e(){var t=this;Object(i.a)(this,e),this.events=void 0,this.length=void 0,this.n=void 0,this.readJson=function(){var e=Object(v.a)(p.a.mark(function e(t){var n,a;return p.a.wrap(function(e){for(;;)switch(e.prev=e.next){case 0:return n="".concat(window.location.origin,"/api/songs/").concat(t,".socool.json"),e.prev=1,e.next=4,O.a.get(n);case 4:return a=e.sent,e.abrupt("return",a.data);case 8:return e.prev=8,e.t0=e.catch(1),e.abrupt("return",{ops:[],length:0});case 11:case"end":return e.stop()}},e,null,[[1,8]])}));return function(t){return e.apply(this,arguments)}}(),this.getPoints=function(e){for(var n=!0,a=[];n;){var r=t.events[t.n];r&&r.t<e?(a.push(r),t.n+=1):n=!1}return a},this.n=0}return Object(s.a)(e,[{key:"getData",value:function(){var e=Object(v.a)(p.a.mark(function e(t){var n;return p.a.wrap(function(e){for(;;)switch(e.prev=e.next){case 0:return e.next=2,this.readJson(t);case 2:n=e.sent,this.events=n.ops,this.length=n.length;case 5:case"end":return e.stop()}},e,this)}));return function(t){return e.apply(this,arguments)}}()}]),e}());Object({NODE_ENV:"production",PUBLIC_URL:""}).PORT;var ce=function e(t){var n=this;Object(i.a)(this,e),this.audio=void 0,this.fadeOut=function(){n.audio.volume>.01?(n.audio.volume-=.01,setTimeout(n.fadeOut,1)):n.audio.pause()},this.play=function(){n.audio.play()},this.pause=function(){n.audio.pause()},this.canPlay=function(){return n.audio.duration};var a="".concat(window.location.origin,"/api/songs/").concat(t,".mp3?").concat(Math.random());this.audio=new Audio(a)};function ie(){var e=Object(j.a)(["\n  position: absolute;\n  background-color: ",";\n  top: 40px;\n  right: 10px;\n"]);return ie=function(){return e},e}function se(){var e=Object(j.a)(["\n  padding-left: 1em;\n  padding-right: 1em;\n  color: white;\n  font-size: 1.25em;\n"]);return se=function(){return e},e}var ue=function(e){function t(e){return Object(i.a)(this,t),Object(u.a)(this,Object(l.a)(t).call(this,e))}return Object(d.a)(t,e),Object(s.a)(t,[{key:"renderStartButton",value:function(){return this.props.play&&this.props.ready?void 0:this.props.ready?r.a.createElement(de,{onClick:this.props.startAnimation,color:"salmon"},r.a.createElement(le,null,"Play")):r.a.createElement(de,{color:"blue"},r.a.createElement(le,null,"Wait."))}},{key:"render",value:function(){return r.a.createElement("div",null,this.renderStartButton())}}]),t}(r.a.Component),le=k.a.p(se()),de=k.a.div(ie(),function(e){return e.color}),he=n(91)(re),fe=150,me=1100,pe=function(e){function t(e){var n;return Object(i.a)(this,t),(n=Object(u.a)(this,Object(l.a)(t).call(this,e))).scene=void 0,n.camera=void 0,n.renderer=void 0,n.geometry=void 0,n.container=void 0,n.controls=void 0,n.t=void 0,n.id=void 0,n.data=void 0,n.audio=void 0,n.startAnimation=Object(v.a)(p.a.mark(function e(){var t;return p.a.wrap(function(e){for(;;)switch(e.prev=e.next){case 0:if(e.prev=0,n.t=Date.now(),!n.audio){e.next=5;break}return e.next=5,n.audio.play();case 5:n.animate(),t=n.data.events[n.data.events.length-1],n.tweenCamera(n.camera,n.controls,t.t,t.l),n.setState(Object(ne.a)({},n.state,{play:!0})),e.next=13;break;case 11:e.prev=11,e.t0=e.catch(0);case 13:case"end":return e.stop()}},e,null,[[0,11]])})),n.renderPoints=function(){var e=(Date.now()-n.t)/1e3,t=n.data.getPoints(e),a=!0,r=!1,o=void 0;try{for(var c,i=t[Symbol.iterator]();!(a=(c=i.next()).done);a=!0){var s=c.value;if(n.data.events.length>0){var u;u=n.createObject(s),n.scene.add(u)}}}catch(l){r=!0,o=l}finally{try{a||null==i.return||i.return()}finally{if(r)throw o}}},n.tweenCamera=function(e,t,a,r){new ae.Tween({position:0}).to({position:n.data.length*fe},1e3*n.data.length).onUpdate(function(){e.position.z=this._object.position+me,t.target.z=this._object.position}).start()},n.t=0,n.container=null,n.id=null,n.audio=null,n.state={play:!1,ready:!1},n}return Object(d.a)(t,e),Object(s.a)(t,null,[{key:"calculateXPos",value:function(e){return-e*window.innerWidth}},{key:"calculateYPos",value:function(e){return 2*e*window.innerHeight-window.innerHeight}},{key:"calculateZPos",value:function(e,t){return e*fe-me+50*t}}]),Object(s.a)(t,[{key:"render",value:function(){var e=this;return r.a.createElement("div",null,r.a.createElement(ue,{ready:this.state.ready,play:this.state.play,startAnimation:this.startAnimation}),r.a.createElement("div",{ref:function(t){e.container=t}}))}},{key:"componentDidMount",value:function(){var e=Object(v.a)(p.a.mark(function e(){var t=this;return p.a.wrap(function(e){for(;;)switch(e.prev=e.next){case 0:return this.setUpThreeJS(),this.setState(Object(ne.a)({},this.state,{ready:!0})),window.addEventListener("resize",this.updateDimensions.bind(this)),window.addEventListener("keydown",function(){var e=Object(v.a)(p.a.mark(function e(n){return p.a.wrap(function(e){for(;;)switch(e.prev=e.next){case 0:if(!t.state.ready||t.state.play||"Space"!==n.code){e.next=4;break}return n.preventDefault(),e.next=4,t.startAnimation();case 4:case"end":return e.stop()}},e)}));return function(t){return e.apply(this,arguments)}}()),e.next=6,this.getData(this.props.song);case 6:!0===this.props.autoplay&&this.startAnimation();case 7:case"end":return e.stop()}},e,this)}));return function(){return e.apply(this,arguments)}}()},{key:"componentWillUnmount",value:function(){var e=Object(v.a)(p.a.mark(function e(){var t=this;return p.a.wrap(function(e){for(;;)switch(e.prev=e.next){case 0:this.audio&&this.audio.fadeOut(),this.id&&window.cancelAnimationFrame(this.id),window.removeEventListener("resize",this.updateDimensions.bind(this)),window.removeEventListener("keydown",function(){var e=Object(v.a)(p.a.mark(function e(n){return p.a.wrap(function(e){for(;;)switch(e.prev=e.next){case 0:if(!t.state.ready||t.state.play||"Space"!==n.code){e.next=4;break}return n.preventDefault(),e.next=4,t.startAnimation();case 4:case"end":return e.stop()}},e)}));return function(t){return e.apply(this,arguments)}}());case 4:case"end":return e.stop()}},e,this)}));return function(){return e.apply(this,arguments)}}()},{key:"componentDidUpdate",value:function(){var e=window.innerWidth,t=window.innerHeight;this.camera.aspect=e/t,this.camera.updateProjectionMatrix(),this.renderer.setSize(e,t,!0)}},{key:"setupScene",value:function(){this.scene.background=new re.Color(4210752)}},{key:"setUpThreeJS",value:function(){this.scene=new re.Scene,this.setupScene(),this.renderer=new re.WebGLRenderer,this.camera=new re.PerspectiveCamera(50,window.innerWidth/window.innerHeight,1,3e4),this.controls=new he(this.camera,this.container),this.camera.lookAt(this.scene.position),this.camera.position.set(0,0,0),this.controls.update(),this.geometry=new re.BoxBufferGeometry(20,20,20),this.renderer.setPixelRatio(window.devicePixelRatio),this.renderer.setSize(window.innerWidth,window.innerHeight),this.renderer.render(this.scene,this.camera),this.container&&this.container.appendChild(this.renderer.domElement)}},{key:"animate",value:function(){this.data.n<this.data.events.length&&this.renderPoints(),this.render(),this.controls.update(),this.renderer.render(this.scene,this.camera),this.id=requestAnimationFrame(this.animate.bind(this)),ae.update()}},{key:"updateDimensions",value:function(){this.camera.aspect=window.innerWidth/window.innerHeight,this.camera.updateProjectionMatrix(),this.renderer.setSize(window.innerWidth,window.innerHeight,!0)}},{key:"getData",value:function(){var e=Object(v.a)(p.a.mark(function e(t){return p.a.wrap(function(e){for(;;)switch(e.prev=e.next){case 0:return this.audio=new ce(t),this.data=new oe,e.next=4,this.data.getData(t);case 4:case"end":return e.stop()}},e,this)}));return function(t){return e.apply(this,arguments)}}()},{key:"createObject",value:function(e){var n={colorA:{type:"vec3",value:new re.Color(e.voice/50*16777215)},colorB:{type:"vec3",value:new re.Color(e.voice/50*16777198)},ta:{type:"f",value:3}},a=new re.ShaderMaterial({fragmentShader:"\n      uniform vec3 colorA; \n      uniform vec3 colorB; \n      // varying vec3 vUv;\n      uniform float time;\n      \n      void main() {\n          vec3 p = mod(\n            gl_FragCoord.xyz, 100.0\n          ); \n          float r = fract(\n            sin(\n              dot(\n                p.xyz ,\n                vec3(12.9898,78.233, 24.3421)\n              )\n            ) * 43758.5453\n          );\n          gl_FragColor = vec4(\n            sin(colorA.r + r), \n            sin(colorA.g + r) - 0.05, \n            // sin(colorA.g + r) - 0.25, \n            sin(colorA.b + r), 0.03\n          );\n      }\n  ",uniforms:n,vertexShader:"\n    // varying vec3 vUv; \n    uniform float time;\n\n    void main() {\n      // vUv = position; \n\n      vec4 modelViewPosition = modelViewMatrix * vec4(position, 1.0) * vec4(1.0, 1.0 + sin(time * position.y), 1.0, 1.0);\n      gl_Position = projectionMatrix * modelViewPosition; \n    }\n  "}),r=new re.Mesh(this.geometry,a),o=e.t*fe-me+50*e.l,c=Math.exp(e.z);return r.position.x=t.calculateXPos(e.x),r.position.y=t.calculateYPos(e.y),r.position.z=t.calculateZPos(e.t,e.l),r.scale.x=1e-5,r.scale.y=1e-5,r.scale.z=1e-5,this.tweenObject(r,e.l,o,c),r}},{key:"tweenObject",value:function(e,t,n,a){new ae.Tween({position:50*t,scale:1.5,scale_z:0}).to({position:0,scale:1,scale_z:7*t},1e3*t).easing(ae.Easing.Sinusoidal.Out).onUpdate(function(){e.scale.x=a*this._object.scale,e.scale.y=a*this._object.scale,e.scale.z=this._object.scale_z,e.position.z=n-this._object.position+2*t}).start()}}]),t}(r.a.Component);function ve(){var e=Object(j.a)(["\n  height: 1.5em;\n  font-size: 1.1em;\n"]);return ve=function(){return e},e}function be(){var e=Object(j.a)(["\n  margin-right: 10px;\n  position: absolute;\n  right: 0;\n  top: 10px;\n"]);return be=function(){return e},e}function ge(){var e=Object(j.a)(["\n  position: absolute;\n  width: 100%;\n  text-align: center;\n"]);return ge=function(){return e},e}function we(){var e=Object(j.a)(["\n  color: #454;\n  font-size: 1.5em;\n"]);return we=function(){return e},e}Object({NODE_ENV:"production",PUBLIC_URL:""}).PORT,k.a.h1(we()),k.a.div(ge());var ye=k.a.div(be()),Oe=k.a.select(ve());var je=function(){var e=Object(f.h)().id;e||(e="");var t=Object(a.useState)(e),n=Object(b.a)(t,2),o=n[0],i=n[1],s=Object(a.useState)([]),u=Object(b.a)(s,2),l=u[0],d=u[1],h=Object(a.useState)(),m=Object(b.a)(h,2),g=m[0],w=m[1],y=Object(f.g)();if(Object({NODE_ENV:"production",PUBLIC_URL:""}).LOCAL){var j=Object(a.useRef)(new WebSocket("ws://127.0.0.1:3012"));Object(a.useEffect)(function(){j.current.onopen=function(){j.current.send("WereSoVisible")},j.current.onmessage=function(e){console.log(e.data),"update"===e.data&&k(o,!0)}}),Object(a.useEffect)(function(){return function(){return j.current.close()}},[j])}var k=function(e,t){g&&(c.a.unmountComponentAtNode(g),c.a.render(r.a.createElement(pe,{song:e,autoplay:t}),g)),i(e),y.push("/play/"+e)},x=function(){var e=Object(v.a)(p.a.mark(function e(){var t,n,a;return p.a.wrap(function(e){for(;;)switch(e.prev=e.next){case 0:return t="".concat(window.location.origin,"/api/songs/song_list.json"),e.next=3,O()(t);case 3:return n=e.sent,e.next=6,n.data.songs;case 6:a=e.sent,d(a);case 8:case"end":return e.stop()}},e)}));return function(){return e.apply(this,arguments)}}();return Object(a.useEffect)(function(){x()},[]),Object(a.useEffect)(function(){k(o,!1)},[g]),r.a.createElement("div",null,r.a.createElement("div",null,r.a.createElement("div",{ref:function(e){w(e)}})),function(e,t,n){return t.length>0?r.a.createElement(ye,null,r.a.createElement(Oe,{onChange:function(e){n(e.target.value,!0)},value:e},t.map(function(e,t){return r.a.createElement("option",{key:t,value:e},e)}))):r.a.createElement(ye,null,r.a.createElement(Oe,{onChange:function(e){},value:"none"},r.a.createElement("option",null,"None")))}(o,l,k))},ke=function(e){function t(){return Object(i.a)(this,t),Object(u.a)(this,Object(l.a)(t).apply(this,arguments))}return Object(d.a)(t,e),Object(s.a)(t,[{key:"render",value:function(){return a.createElement(h.a,null,a.createElement(f.d,null,a.createElement(f.b,{path:"/compose",children:a.createElement(te,null)}),a.createElement(f.b,{path:"/play/:id",children:a.createElement(je,null)}),a.createElement(f.a,{from:"/",to:"/play/table"})))}}]),t}(a.Component);n(93),Boolean("localhost"===window.location.hostname||"[::1]"===window.location.hostname||window.location.hostname.match(/^127(?:\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}$/));c.a.render(r.a.createElement(ke,null),document.getElementById("root")),"serviceWorker"in navigator&&navigator.serviceWorker.ready.then(function(e){e.unregister()})}},[[54,1,2]]]);
//# sourceMappingURL=main.ca72c2c6.chunk.js.map