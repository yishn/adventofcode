// Part 1
document.body.innerText.split`\n`.map(l=>(n=l.split`\t`,Math.max(...n)-Math.min(...n))).reduce((s,x)=>s+x)

// Part 2
document.body.innerText.split`\n`.map(l=>l.split`\t`.map((x,_,n)=>n.find(y=>y>+x&y%x==0)/x).find(x=>!isNaN(x))||0).reduce((s,x)=>s+x)
