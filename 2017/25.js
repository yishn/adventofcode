// Part 1
for(I=document.body.innerText.split`\n`,s=I[0][15],[t]=I[1].match(/\d+/),S={},i=0;i<6;i++)S[I[3+i*10][9]]=[5,9].map(j=>I.slice(j+i*10,j+3+i*10).map(x=>x.split` `.slice(-1)[0][0]));for(T={},p=i=0;i<t;i++)[w,d,s]=S[s][T[p]|0],T[p]=+w,d=='r'?p++:p--;Object.values(T).reduce((s,x)=>s+x)
