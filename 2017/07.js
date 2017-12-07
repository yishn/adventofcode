// Part 1
I=document.body.innerText.split`\n`,I.map(x=>x.split` `[0]).find(w=>!I.some(e=>[...e.split`>`,''][1].includes(w)))

// Part 2
P={},document.body.innerText.split`\n`.map(l=>l&&(i=l.indexOf('>'),P[l.split` `[0]]=[+l.match(/\d+/)[0],i>0?l.slice(i+2).split`, `:[]])),v=w=>P[w][1].reduce((s,x)=>s+v(x),P[w][0]),f=w=>P[w][1].find((x,_,c)=>!c.some(y=>x!=y&v(x)==v(y))),F=Object.keys(P).map(f),x=F.find(w=>w&&!f(w)),P[x][0]+v(P[F.find(w=>w&&P[w][1].includes(x))][1].find(w=>w!=x))-v(x)
