// Part 1
[...document.body.innerText.trim()].reduce((s,c,i,a)=>(c==a[(i+1)%a.length])*c+s,0)

// Part 2
[...document.body.innerText.trim()].reduce((s,c,i,a)=>(l=a.length,c==a[(i+l/2)%l])*c+s,0)
