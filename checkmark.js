document.getElementById('fileInput').addEventListener('change', function() {
    let file = this.files[0];
    let reader = new FileReader();
    reader.onload = function() {
        let data = JSON.parse(this.result);
        let table = document.getElementById('table');
        table.innerHTML = '';
        let thead = document.createElement('thead');
        let tr = document.createElement('tr');
        for (let key in data[0]) {
            let th = document.createElement('th');
            th.textContent = key;
            tr.appendChild(th);
        }
        thead.appendChild(tr);
        table.appendChild(thead);
        let tbody = document.createElement('tbody');
        for (let i = 0; i < data.length; i++) {
            let tr = document.createElement('tr');
            if (data[i].red) {
                tr.style.backgroundColor = 'red';
            }
            for (let key in data[i]) {
                let td = document.createElement('td');
                if (key === 'red') {
                    let input = document.createElement('input');
                    input.type = 'checkbox';
                    input.checked = data[i][key];
                    input.addEventListener('change', function() {
                        tr.style.backgroundColor = this.checked ? 'red' : '';
                        data[i][key] = this.checked;
                    });
                    td.appendChild(input);
                } else if (key === 'comment') {
                    let textarea = document.createElement('textarea');
                    textarea.value = data[i][key] || '';
                    textarea.addEventListener('change', function() {
                        data[i][key] = this.value;
                    });
                    td.appendChild(textarea);
                } else {
                    td.textContent = data[i][key];
                }
                tr.appendChild(td);
            }
            tbody.appendChild(tr);
        }
        table.appendChild(tbody);

        let saveButton = document.createElement('button');
        saveButton.textContent = 'Save';
        saveButton.addEventListener('click', function() {
            let json = JSON.stringify(data, null, 2);
            let blob = new Blob([json], {type: 'application/json'});
            let url = URL.createObjectURL(blob);
            let a = document.createElement('a');
            a.href = url;
            a.download = 'updated_file_info.json';
            a.click();
            URL.revokeObjectURL(url);
        });
        table.parentNode.insertBefore(saveButton, table.nextSibling);
    };
    reader.readAsText(file);
});
