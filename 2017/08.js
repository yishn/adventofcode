// Part 1
R={},document.body.innerText.split`\n`.map(l=>l&&([x,a,y,,z,o,v]=l.split` `,R[x]=(R[x]|0)+y*(a[0]=='i'?1:-1)*eval((R[z]|0)+o+v))),Math.max(...Object.values(R))
