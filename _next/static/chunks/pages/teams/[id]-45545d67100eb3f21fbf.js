(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[661],{9102:function(t,e,r){"use strict";r.d(e,{Z:function(){return s}});var n=r(9034),i=r.n(n),a=r(5893),s=function(){return(0,a.jsx)("div",{style:{height:"100%",justifyContent:"center",alignItems:"center"},children:(0,a.jsx)("div",{style:{maxWidth:"960px",margin:"0 auto"},children:(0,a.jsx)("div",{className:i()["lds-dual-ring"]})})})}},8256:function(t,e,r){"use strict";var n=r(3154),i=r(8243),a=r(2503);e.Z=function(t){return i.s?(0,n.s)(t):(0,a.ZP)("/scoreboard")}},359:function(t,e,r){"use strict";r.d(e,{v:function(){return o}});var n=r(7484),i=r.n(n),a=r(285),s=r.n(a);i().extend(s());var o=function(t){return i()(1e3*t).format("YYYY-MM-DD HH:mm:ss Z")}},9510:function(t,e,r){"use strict";r.r(e),r.d(e,{__N_SSG:function(){return _},default:function(){return g}});var n=r(9102),i=r(8256),a=r(3154),s=r(8243),o=r(2503),c=function(t,e){return s.s?(0,a.s)(e):(0,o.ZP)("/team/".concat(t))},u=r(2809);var l=r(6988);function f(t,e){return function(t){if(Array.isArray(t))return t}(t)||function(t,e){var r=null==t?null:"undefined"!==typeof Symbol&&t[Symbol.iterator]||t["@@iterator"];if(null!=r){var n,i,a=[],s=!0,o=!1;try{for(r=r.call(t);!(s=(n=r.next()).done)&&(a.push(n.value),!e||a.length!==e);s=!0);}catch(c){o=!0,i=c}finally{try{s||null==r.return||r.return()}finally{if(o)throw i}}return a}}(t,e)||(0,l.Z)(t,e)||function(){throw new TypeError("Invalid attempt to destructure non-iterable instance.\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.")}()}var d=r(6486),m=r(359),h=r(7048),p=r(4976),j=r(3551),y=r.n(j),v=r(5893);function x(t,e){var r=Object.keys(t);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(t);e&&(n=n.filter((function(e){return Object.getOwnPropertyDescriptor(t,e).enumerable}))),r.push.apply(r,n)}return r}var b=function(t){var e=t.team,r=t.scorefeed,n=t.series,i=(0,d.orderBy)(Object.entries(r.taskStats).map((function(t){var e=f(t,2);return function(t){for(var e=1;e<arguments.length;e++){var r=null!=arguments[e]?arguments[e]:{};e%2?x(Object(r),!0).forEach((function(e){(0,u.Z)(t,e,r[e])})):Object.getOwnPropertyDescriptors?Object.defineProperties(t,Object.getOwnPropertyDescriptors(r)):x(Object(r)).forEach((function(e){Object.defineProperty(t,e,Object.getOwnPropertyDescriptor(r,e))}))}return t}({name:e[0]},e[1])})),["time"],["desc"]);return(0,v.jsxs)(v.Fragment,{children:[(0,v.jsxs)("h1",{children:[e.country?(0,v.jsx)(h.Z,{country:e.country}):"",e.teamname,(0,v.jsxs)("span",{className:y()["team-info"],children:["Rank ",r.pos," / ",r.score," points"]})]}),(0,v.jsx)(p.Z,{chartTeams:[e.teamname],chartSeries:n}),r&&(0,v.jsxs)("table",{className:y()["score-table"],children:[(0,v.jsx)("thead",{children:(0,v.jsxs)("tr",{children:[(0,v.jsx)("th",{children:"Task"}),(0,v.jsx)("th",{children:"Score"}),(0,v.jsx)("th",{children:"Solved At"})]})}),(0,v.jsx)("tbody",{children:i.map((function(t){return(0,v.jsxs)("tr",{children:[(0,v.jsx)("td",{children:t.name}),(0,v.jsx)("td",{children:t.points}),(0,v.jsx)("td",{children:(0,m.v)(t.time)})]},t.name)}))})]})]})},_=!0,g=function(t){var e=t.team,r=t.scoreboard,a=t.series,s=c(e.team_id.toString(),e).data,o=(0,i.Z)(r).data;if(!s||!o)return(0,v.jsx)(n.Z,{});var u=o.filter((function(t){return t.team_id===s.team_id}))[0];return b({team:s,scorefeed:u,series:a})}},4976:function(t,e,r){"use strict";var n=r(7809),i=r(5893);e.Z=function(t){var e=t.chartTeams,r=t.chartSeries.map((function(t,r){return{name:e[r],type:"line",showSymbol:!1,data:t.map((function(t){return[1e3*t.time,t.score]}))}})),a=e.map((function(t){return{name:t}}));return(0,i.jsx)(n.Z,{option:{tooltip:{trigger:"axis",axisPointer:{type:"cross",animation:"false"}},legend:{data:a},xAxis:{type:"time"},yAxis:{type:"value"},series:r},notMerge:!0})}},7048:function(t,e,r){"use strict";var n=r(3023),i=r(5893);e.Z=function(t){var e=t.country,r=n.P5.countries({alpha2:e})[0];return r?(0,i.jsx)("span",{title:r.name,children:r.emoji}):(0,i.jsx)(i.Fragment,{})}},5175:function(t,e,r){(window.__NEXT_P=window.__NEXT_P||[]).push(["/teams/[id]",function(){return r(9510)}])},9034:function(t){t.exports={"lds-dual-ring":"loading_lds-dual-ring__1T8NG"}},3551:function(t){t.exports={"team-info":"team_team-info__2rrGJ","score-table":"team_score-table__3AFlE"}}},function(t){t.O(0,[662,562,774,888,179],(function(){return e=5175,t(t.s=e);var e}));var e=t.O();_N_E=e}]);