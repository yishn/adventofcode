// Part 1
I=+document.querySelector('.puzzle-input').innerText,w=Array(I).findIndex((_,i)=>i%2>0&i**2>I)-1,x=y=0,[...Array((w+1)**2-I)].map((_,i)=>(x+=i<w?1:i>=2*w&i<3*w?-1:0,y+=i>=3*w?-1:i>=w&i<2*w?1:0)),Math.abs(w/2-x)+Math.abs(w/2-y)
