$(document).ready(function() {
    showBudgetsForClassification('wants');

    $('.transaction-details__dropdown').on('click', function() {
        let budget = $(this).parents('.budget');
        let transactionDetailsContainer = budget.find('.transaction-details__container');
        let budgetId = budget.data('id');
        let icon = $(this).find('i');

        if (transactionDetailsContainer.is(':visible')) {
            icon.removeClass('fa-chevron-down').addClass('fa-chevron-right');
            transactionDetailsContainer.hide();
            return;
        }

        icon.removeClass('fa-chevron-right').addClass('fa-chevron-down');
        transactionDetailsContainer.show();
        transactionDetailsContainer.empty().append('<i class="fa-solid fa-spinner fa-spin"></i>');

        $.get('/secure/api/transaction-group/top-by-budget/' + budgetId + '/5/' + window._year + '/' + window._month, function(transactionGroups) {
            console.log(transactionGroups);

            let list = $('<ul class="transaction-details__list"></ul>');

            $.each(transactionGroups, function(index, transactionGroup) {
                let transactionDetails = $('<li class="transaction-details"></li>');
                let title = $('<div class="transaction-details__description"></div>')
                    .text(transactionGroup.bank_description + (transactionGroup.count > 1 ? ' (' + transactionGroup.count + ' transactions)' : ''));

                let amount = $('<div class="transaction-details__amount"></div>')
                    .text(formatAmount(transactionGroup.total_amount));

                if (transactionGroup.total_amount < 0) {
                    amount.addClass('transaction-details__amount--negative');
                } else {
                    amount.addClass('transaction-details__amount--positive');
                }

                transactionDetails.append(title).append(amount);

                list.append(transactionDetails);
            });

            if (transactionGroups.length == 0) {
                let message = $('<li class="transaction-details"></li>').text('No transactions');
                list.append(message);
            }

            transactionDetailsContainer.empty().append(list);
        });
    });

    $('.classification-select__entry').on('click', function() {
        $('.classification-select__entry').removeClass('classification-select__entry--selected');
        $(this).addClass('classification-select__entry--selected');

        let classification = $(this).data('classification');
        showBudgetsForClassification(classification);
    });

    function showBudgetsForClassification(classification) {
        $('.budget').not('.budget--total').hide();
        switch (classification) {
            case 'wants':
                $('.budget--wants').show();
                break;
            case 'needs':
                $('.budget--needs').show();
                break;
            case 'savings':
                $('.budget--savings').show();
                break;
            default:
                console.error('Unknown classification: ', classification);
        }
    }

    function formatAmount(amount) {

        let negative = amount < 0;
        let dollars = Math.abs(Math.floor(amount / 100));
        let cents = Math.abs(amount % 100);

        let output = negative ? "-$" : "$";
        output += dollars.toLocaleString();
        output += "." + (cents < 10 ? "0" + cents : cents);

        return output;
    }
});