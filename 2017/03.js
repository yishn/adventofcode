// Part 1

I=document.querySelector('.puzzle-input').innerText,w=Array(+I).findIndex((_,i)=>i%2>0&i**2>I)-1,x=0,y=0,[...Array(4*w)].map((_,i)=>(d=(w+1)**2-i>I,x+=d*(i<w?1:i>=2*w&i<3*w?-1:0),y+=d*(i>=3*w?-1:i>=w&i<2*w?1:0))),Math.abs(w/2-x)+Math.abs(w/2-y)
