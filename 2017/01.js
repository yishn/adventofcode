// Part 1
[...document.body.textContent.trim()].reduce((s,c,i,a)=>+(c==a[(i+1)%a.length])*+c+s,0)

// Part 2
[...document.body.textContent.trim()].reduce((s,c,i,a)=>+(c==a[(i+a.length/2)%a.length])*+c+s,0)
