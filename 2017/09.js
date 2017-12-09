// Part 1
(s=(o,l=1)=>!o?0:o.reduce((a,x)=>a+s(x,l+1),l))(JSON.parse(document.body.innerText.replace(/{|}|<([^!>]|!.)*>/g,x=>x=='{'?'[':x=='}'?']':'0')))

// Part 2
s=0,document.body.innerText.replace(/<([^!>]|!.)*/g,x=>s+=x.replace(/!./g,'').length-1),s
