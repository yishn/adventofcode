// Part 1
I=document.body.innerText.split`\n`,I.map(x=>x.split` `[0]).find(w=>!I.some(e=>[...e.split`>`,''][1].includes(w)))
