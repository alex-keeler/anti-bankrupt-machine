$(document).ready(function() {

    $.get('/secure/api/cash-flow/data/' + window._year + '/' + window._month, function(cashFlowData) {
        console.log(cashFlowData);
        buildBudgetStackedBarChart(cashFlowData);
    });
});

function buildBudgetStackedBarChart(cashFlowData) {

    let incomeBudgetChartContext = $('#income-budget-chart');

    let datasets = [];
    let colors = ['green', 'red', 'blue', 'orange', 'yellow', 'purple'];

    $(cashFlowData.budget_state_list).each(function(index, budgetState) {

        let data = [0, 0, 0];

        if (budgetState.classification === 'Income') {
            data[0] = budgetState.amount_contributed_this_month / 100;
        } else {
            data[1] = budgetState.amount_monthly / 100;
            data[2] = (budgetState.amount_spent_this_month + budgetState.amount_contributed_this_month) / 100;
        }

        let dataset = {
            label: budgetState.title,
            data: data,
            backgroundColor: colors[index % colors.length]
        };
        datasets.push(dataset);
    });
    //
    // $(cashFlowData.budget_details_list).each(function(index, budget) {
    //
    //     let budgetedDataset = {
    //         label: budget.title,
    //         data: [0, budget.amount_monthly / 100, 0],
    //         backgroundColor: colors[index % colors.length]
    //     }
    //
    //     let spentDataset = {
    //         label: budget.title,
    //         data: [0, 0, (Math.max(0, budget.amount_spent - budget.amount_contributed) + budget.amount_contributed_month) / 100],
    //         backgroundColor: colors[index % colors.length]
    //     }
    //
    //     datasets.push(budgetedDataset);
    //     datasets.push(spentDataset);
    // });

    let spentUnbudgetedDataset = {
        label: "Unbudgeted",
        data: [0, 0, cashFlowData.amount_spent_unbudgeted / 100],
        backgroundColor: 'gray'
    }

    datasets.push(spentUnbudgetedDataset);

    let incomeBudgetChart = new Chart(incomeBudgetChartContext, {
        type: 'bar',
        data: {
            labels: ['Income', 'Budgeted', 'Spent'],
            datasets: datasets
        },
        options: {
            plugins: {
                title: {
                    display: true,
                    text: 'Income & Budgets'
                },
                legend: {
                    display: false
                }
            },
            scales: {
                x: {
                    stacked: true
                },
                y: {
                    stacked: true
                }
            },
            maintainAspectRatio: false,
        }
    });
}