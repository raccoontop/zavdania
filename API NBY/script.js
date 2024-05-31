document.addEventListener('DOMContentLoaded', function () {
    const exchangeRatesTableBody = document.querySelector('#exchange-rates tbody');

    fetch('https://bank.gov.ua/NBUStatService/v1/statdirectory/exchange?json')
        .then(response => response.json())
        .then(data => {
            data.forEach(currency => {
                const row = `<tr><td>${currency.cc} (${currency.txt})</td><td>${currency.rate}</td></tr>`;
                exchangeRatesTableBody.innerHTML += row;
            });
        })
        .catch(error => {
            exchangeRatesTableBody.innerHTML = `<tr><td colspan="2">Помилка отримання даних: ${error}</td></tr>`;
            console.error('Помилка:', error);
        });
});

