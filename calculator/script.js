document.addEventListener('DOMContentLoaded', function() {
    const display = document.getElementById('display');
    const buttons = document.querySelectorAll('.btn');
    let currentInput = '';
    let operator = null;
    let previousInput = '';

    function updateDisplay(value) {
        display.textContent = value;
    }

    buttons.forEach(button => {
        button.addEventListener('click', function() {
            const action = this.getAttribute('data-action');

            if (!isNaN(action) || action === '.') {
                if (action === '.' && currentInput.includes('.')) return;
                currentInput += action;
                updateDisplay(currentInput);
            } else if (action === 'clear') {
                currentInput = '';
                previousInput = '';
                operator = null;
                updateDisplay('0');
            } else if (action === 'backspace') {
                currentInput = currentInput.slice(0, -1);
                updateDisplay(currentInput || '0');
            } else if (action === 'equals') {
                if (previousInput && operator) {
                    currentInput = eval(previousInput + operator + currentInput).toString();
                    updateDisplay(currentInput);
                    previousInput = '';
                    operator = null;
                }
            } else if (action === 'percent') {
                if (currentInput) {
                    currentInput = (parseFloat(currentInput) / 100).toString();
                    updateDisplay(currentInput);
                }
            } else {
                if (currentInput) {
                    if (previousInput && operator) {
                        currentInput = eval(previousInput + operator + currentInput).toString();
                        updateDisplay(currentInput);
                    }
                    previousInput = currentInput;
                    currentInput = '';
                }
                operator = {
                    'add': '+',
                    'subtract': '-',
                    'multiply': '*',
                    'divide': '/'
                }[action];
            }
        });
    });
});
