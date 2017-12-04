// Part 1
I=document.querySelector('.puzzle-input').innerText,w=~~Math.sqrt(I),w+=w%2;for(x=y=i=0;i<(w+1)**2-I;i++){x+=i<w?1:i>=2*w&i<3*w?-1:0;y+=i>=3*w?-1:i>=w&i<2*w?1:0}Math.abs(w/2-x)+Math.abs(w/2-y)

// Part 2
for(I=document.querySelector('.puzzle-input').innerText,S={'0,0':1},x=y=w=0;w<I;w+=2){for(i=0;i<4*w&S[[x,y]]<=I;i++){x+=!i|i>=3*w?1:i>=w&i<2*w?-1:0;y+=i>0&i<w?-1:i>=2*w&i<3*w?1:0;S[[x,y]]=[-1,0,1].reduce((s,d,_,a)=>a.reduce((s,e)=>s+(S[[x+d,y+e]]||0),s),0)}}S[[x,y]]
