// Part 1
document.body.innerText.split`\n`.filter(l=>!l.split` `.some((w,i,a)=>a.some((v,j)=>w==v&i!=j))).length-1

// Part 2
document.body.innerText.split`\n`.filter(l=>!l.split` `.map(x=>[...x].sort().join()).some((w,i,a)=>a.some((v,j)=>w==v&i!=j))).length-1
