// Part 1
I=document.querySelector('.puzzle-input').innerText,w=~~Math.sqrt(I),w+=w%2,x=y=0,[...Array((w+1)**2-I)].map((_,i)=>(x+=i<w?1:i>=2*w&i<3*w?-1:0,y+=i>=3*w?-1:i>=w&i<2*w?1:0)),Math.abs(w/2-x)+Math.abs(w/2-y)

// Part 2
I=document.querySelector('.puzzle-input').innerText,S={'0,0':1},x=y=0,[...Array(~~Math.sqrt(I))].map((_,w)=>w>1&w%2<1&&[...Array(4*w)].map((_,i)=>S[[x,y]]<=I&&(x+=!i|i>=3*w?1:i>=w&i<2*w?-1:0,y+=i>0&i<w?-1:i>=2*w&i<3*w?1:0,S[[x,y]]=[-1,0,1].reduce((s,d,_,a)=>a.reduce((s,e)=>s+(S[[x+d,y+e]]||0),s),0)))),S[[x,y]]
