
$(document).ready(function () {

    setupBudgetMenu();

    $('.transaction-list__user-description').each(function () {
        setupUserDescriptionElement($(this));
    });

    $('.transaction-list__comment').each(function () {
        setupCommentElement($(this));
    });

    $('.budget-mapping__container').each(function () {
        setupBudgetMappings($(this));
    });

    $(window).bind('beforeunload', function () {
        let currentlyEditingTransaction = false;

        $('.budget-mapping__container').each(function() {
            if ($(this).data('editing')) {
                currentlyEditingTransaction = true;
                return false; // break
            }
        })

        if (currentlyEditingTransaction) {
            return 'You are still editing budget mappings on a transaction. Are you sure you want to leave?';
        }
    });
});

function setupBudgetMappings(budgetMappingContainer) {

    let budgetMappings = $(budgetMappingContainer).find('.budget-mapping');
    let addButton = $(budgetMappingContainer).find('.budget-mapping__add');
    let editButton = $(budgetMappingContainer).find('.budget-mapping__edit');
    let saveButton = $(budgetMappingContainer).find('.budget-mapping__save');

    $(budgetMappings).each(function () {
        setupBudgetMapping($(this));
    });

    $(addButton).on('click', function () {
        addBudgetMapping(budgetMappingContainer);
    });

    $(editButton).on('click', function () {
        editBudgetMappings(budgetMappingContainer);
    });

    $(saveButton).on('click', function () {
        saveBudgetMappings(budgetMappingContainer);
    });
}

function setupBudgetMapping(budgetMappingElement) {

    let categoryElement = $(budgetMappingElement).find('.budget-mapping__category');
    let budgetMappingContainer = $(budgetMappingElement).parents('.budget-mapping__container');
    let amountElement = $(budgetMappingElement).find('.budget-mapping__amount');
    let behaviorElement = $(budgetMappingElement).find('.budget-mapping__behavior');

    if ($(budgetMappingElement).hasClass('budget-mapping--none')) {

        $(categoryElement).on('mouseenter', function () {
            openBudgetMenu($(this));
        });

        $(categoryElement).on('mouseleave', function () {
            if ($('#budget-select:hover').length == 0) {
                $('#budget-select').hide();
                $('#budget-select').find('.menu').hide();
            }
        });
    } else {
        $(categoryElement).off('mouseenter');
        $(categoryElement).off('mouseleave');

        $(categoryElement).find('.budget-mapping__remove').off('click').on('click', function () {
            let mapping = $(this).parents('.budget-mapping');
            let mappingId = mapping.data('id');

            $.post('/secure/api/budget-transaction-mapping/remove/' + mappingId)
                .done(function (transactionDetails) {
                    updateBudgetMappingElement(mapping, transactionDetails);
                })
                .fail(function () {
                    alert('An error has occurred.');
                });
        });

        let behaviorIcon;
        switch ($(budgetMappingElement).data('behavior')) {
            case 'Spending':
                behaviorIcon = $('<i class="fa-solid fa-credit-card" title="Spending"></i>');
                break;
            case 'Contribution':
                behaviorIcon = $('<i class="fa-solid fa-piggy-bank" title="Contribution"></i>');
                break;
            case 'Reimbursement':
                behaviorIcon = $('<i class="fa-solid fa-money-bill-transfer" title="Reimbursement"></i>');
                break;
        }
        $(behaviorElement).html(behaviorIcon);

        if ($(budgetMappingContainer).data('editing') && $(amountElement).find('input').length == 0) {
            let amountText = $(amountElement).text();
            let amountInput = $('<input type="number" step="0.01" name="amount">');
            amountInput.val(amountText.replace('$', '').replace(',', ''));
            $(amountElement).text('$');
            $(amountElement).append(amountInput);
            $(amountInput).on('change', function () {
                $(this).val(parseFloat($(this).val()).toFixed(2));
                if ($(this).val() > 0 && $(budgetMappingElement).data('behavior') == 'Spending') {
                    $(budgetMappingElement).data('behavior', 'Contribution');
                    setupBudgetMapping(budgetMappingElement);
                } else if ($(this).val() <= 0 && $(budgetMappingElement).data('behavior') != 'Spending') {
                    $(budgetMappingElement).data('behavior', 'Spending');
                    setupBudgetMapping(budgetMappingElement);
                }
            });

            $(behaviorElement).css('cursor', 'pointer');
            $(behaviorElement).on('click', function () {
                let newBehavior;
                switch ($(budgetMappingElement).data('behavior')) {
                    case 'Spending':
                        newBehavior = 'Spending';
                        break;
                    case 'Contribution':
                        newBehavior = 'Reimbursement';
                        break;
                    case 'Reimbursement':
                        newBehavior = 'Contribution';
                        break;
                }
                $(budgetMappingElement).data('behavior', newBehavior);
                setupBudgetMapping(budgetMappingElement);
            });
        } else if (!($(budgetMappingContainer).data('editing')) && $(amountElement).find('input').length > 0) {
            let amountInput = $(amountElement).find('input');
            let amountValue = Math.round($(amountInput).val() * 100);
            $(amountElement).empty().text(formatAmount(amountValue));

            $(behaviorElement).css('cursor', 'default');
            $(behaviorElement).off('click');
        }
    }
}

function setupUserDescriptionElement(userDescriptionElement) {
    let text = $(userDescriptionElement).find('.transaction-list__user-description--text');
    let link = $(userDescriptionElement).find('.transaction-list__user-description--link');
    let input = $(userDescriptionElement).find('.transaction-list__user-description--input');

    $(input).on('keypress', function (e) {
        if (e.keyCode === 13) {
            updateUserDescription(userDescriptionElement);
        }
    });

    $(input).on('focusout', function () {
        updateUserDescription(userDescriptionElement);
    });

    $(text).on('click', function () {
        input.val(text.text());
        showUserDescriptionElement(userDescriptionElement, true);
    });

    $(link).on('click', function () {
        input.val("");
        showUserDescriptionElement(userDescriptionElement, true);
    });

    showUserDescriptionElement(userDescriptionElement, false);
}

function updateUserDescription(userDescriptionElement) {
    let transactionId = $(userDescriptionElement).parents('.transaction-list__entry').data('id');
    let userDescription = $(userDescriptionElement).find('.transaction-list__user-description--input').val();
    $.post('/secure/api/transaction/set-user-description/' + transactionId, userDescription, function (transactionDetails) {
        $(userDescriptionElement).find('.transaction-list__user-description--text').text(userDescription);
        showUserDescriptionElement(userDescriptionElement, false);
    });
}

function showUserDescriptionElement(userDescriptionElement, editing) {
    let text = $(userDescriptionElement).find('.transaction-list__user-description--text');
    let link = $(userDescriptionElement).find('.transaction-list__user-description--link');
    let input = $(userDescriptionElement).find('.transaction-list__user-description--input');

    if (editing) {
        input.show();
        text.hide();
        link.hide();
        input.focus();
    } else if (text.text()) {
        text.show();
        link.hide();
        input.hide();
    } else {
        link.show();
        text.hide();
        input.hide();
    }
}

function setupCommentElement(commentElement) {
    let text = $(commentElement).find('.transaction-list__comment--text');
    let link = $(commentElement).find('.transaction-list__comment--link');
    let input = $(commentElement).find('.transaction-list__comment--input');

    $(input).on('keypress', function (e) {
        if (e.keyCode === 13) {
            updateComment(commentElement);
        }
    });

    $(input).on('focusout', function () {
        updateComment(commentElement);
    });

    $(text).on('click', function () {
        input.val(text.text());
        showCommentElement(commentElement, true);
    });

    $(link).on('click', function () {
        input.val("");
        showCommentElement(commentElement, true);
    });

    showCommentElement(commentElement, false);
}

function updateComment(commentElement) {
    let transactionId = $(commentElement).parents('.transaction-list__entry').data('id');
    let comment = $(commentElement).find('.transaction-list__comment--input').val();
    $.post('/secure/api/transaction/set-comment/' + transactionId, comment, function (transactionDetails) {
        $(commentElement).find('.transaction-list__comment--text').text(comment);
        showCommentElement(commentElement, false);
    });
}

function showCommentElement(commentElement, editing) {
    let text = $(commentElement).find('.transaction-list__comment--text');
    let link = $(commentElement).find('.transaction-list__comment--link');
    let input = $(commentElement).find('.transaction-list__comment--input');

    if (editing) {
        input.show();
        text.hide();
        link.hide();
        input.focus();
    } else if (text.text()) {
        text.show();
        link.hide();
        input.hide();
    } else {
        link.show();
        text.hide();
        input.hide();
    }
}

function setupBudgetMenu() {
    let topLevelMenu = $('#budget-select');

    $('.menu__item').on('mouseenter', function () {
        let menuItem = $(this);
        let menu = menuItem.parent();
        $('#budget-select').find('.menu').not(menu).hide();
        let submenu = menuItem.find('.menu');
        if (submenu.length != 0) {
            let left = menu.position().left - submenu.outerWidth();
            submenu.css('left', left + 'px');
            let top = menu.position().top + menuItem.position().top;
            let pageBottom = $(window).height() + $(window).scrollTop();
            if (top + submenu.height() > pageBottom) {
                top = pageBottom - submenu.height();
            }
            submenu.css('top', top + "px");
            submenu.show();
            submenu.show();
        }
    });

    topLevelMenu.on('mouseleave', function () {
        topLevelMenu.hide();
        topLevelMenu.find('.menu').hide();
    });
}

function openBudgetMenu(categoryElement) {

    let menu = $('#budget-select');
    let budgetMappingElement = $(categoryElement).parent('.budget-mapping');
    let transactionEntry = budgetMappingElement.parents('.transaction-list__entry');

    let top = budgetMappingElement.offset().top + $(window).scrollTop() + budgetMappingElement.outerHeight();
    let pageBottom = $(window).scrollTop() + $(window).height();
    if (top + menu.height() > pageBottom) {
        top = pageBottom - menu.height();
    }

    menu.css('right', '75px');
    menu.css('top', top + 'px');
    menu.show();

    $('.menu__item').off('click').on('click', function (e) {
        e.stopPropagation();

        let transactionId = transactionEntry.data('id');
        let transactionAmount = transactionEntry.data('amount');
        let allMappings = transactionEntry.find('.budget-mapping');
        let budgetId = $(this).data('id');

        let amount = allMappings.length > 1 ? 0 : parseInt(transactionAmount);
        let behavior = transactionAmount < 0 ? 'Spending' : 'Contribution';

        $.post('/secure/api/budget-transaction-mapping/add/' + transactionId + '/' + budgetId + '?amount=' + amount + '&behavior=' + behavior)
            .done(function (mappingDetails) {
                updateBudgetMappingElement(budgetMappingElement, mappingDetails);
            });
    });
}

function updateBudgetMappingElement(budgetMappingElement, mappingDetails) {

    let menu = $('#budget-select');

    menu.hide();
    menu.find('.menu').hide();

    let budgetMappingContainer = budgetMappingElement.parents('.budget-mapping__container');

    if (mappingDetails.budget_id) {
        budgetMappingElement.removeClass('budget-mapping--none');
        budgetMappingElement.attr('data-id', mappingDetails.id);

        let behavior = $('<div class="budget-mapping__behavior"></div>');
        let amount = $('<div class="budget-mapping__amount">' + formatAmount(mappingDetails.amount) + '</div>');
        let category = $('<div class="budget-mapping__category"></div>')
        let textElem = $('<span/>').text(mappingDetails.budget_title);
        let removeIcon = $('<span class="budget-mapping__remove"><i class="fa-solid fa-xmark"></i></span>');
        category.append(textElem, removeIcon);

        budgetMappingElement.empty().append(behavior, amount, category);
        budgetMappingElement.attr('data-behavior', mappingDetails.behavior);
        setupBudgetMapping(budgetMappingElement);

        if ($(budgetMappingContainer).find('.budget-mapping').length > 1) {
            editBudgetMappings(budgetMappingContainer);
        }
    } else {
        let transactionEntry = budgetMappingElement.parents('.transaction-list__entry');
        budgetMappingElement.remove();

        let remainingMappings = transactionEntry.find('.budget-mapping');
        if (remainingMappings.length == 0) {
            $(budgetMappingContainer).data('editing', false);
            $(budgetMappingContainer).find('.budget-mapping__edit').show();
            $(budgetMappingContainer).find('.budget-mapping__save').hide();
            addBudgetMapping(budgetMappingContainer);
        } else {
            editBudgetMappings(budgetMappingContainer);
        }
    }
}

function addBudgetMapping(budgetMappingContainer) {
    let budgetMapping = $('<li class="budget-mapping budget-mapping--none"></li>');
    let category = $('<div class="budget-mapping__category"><span>Uncategorized</span></div>');
    $(budgetMapping).append(category);
    $(budgetMappingContainer).find('.budget-mapping__list').append(budgetMapping);
    setupBudgetMapping(budgetMapping);
}

function editBudgetMappings(budgetMappingContainer) {

    $(budgetMappingContainer).data('editing', true);

    let budgetMappings = $(budgetMappingContainer).find('.budget-mapping');
    $(budgetMappings).each(function () {
        setupBudgetMapping($(this));
    });

    let editButton = $(budgetMappingContainer).find('.budget-mapping__edit');
    let saveButton = $(budgetMappingContainer).find('.budget-mapping__save');

    $(editButton).hide();
    $(saveButton).show();
}

function saveBudgetMappings(budgetMappingContainer) {

    if (!validateBudgetMappings(budgetMappingContainer)) {
        return;
    }

    let budgetMappings = $(budgetMappingContainer).find('.budget-mapping');

    $(budgetMappings).each(function() {

        let budgetMapping = this;
        if ($(budgetMapping).hasClass('budget-mapping--none')) {
            return true; // continue
        }

        let amountElement = $(budgetMapping).find('.budget-mapping__amount');
        let amountInput = $(amountElement).find('input');

        let mappingId = $(budgetMapping).data('id');
        let amount = Math.round(amountInput.val() * 100);
        let behavior = $(budgetMapping).data('behavior');

        $.post('/secure/api/budget-transaction-mapping/update/' + mappingId + '?amount=' + amount + '&behavior=' + behavior)
            .done(function(data) {
                console.log('mapping update complete');
            })
            .fail(function(error) {
                alert('An error has occurred. Could not update budget mappings.');
            });
    });

    $(budgetMappingContainer).data('editing', false);

    $(budgetMappings).each(function() {
       setupBudgetMapping($(this));
    });

    let editButton = $(budgetMappingContainer).find('.budget-mapping__edit');
    let saveButton = $(budgetMappingContainer).find('.budget-mapping__save');

    $(editButton).show();
    $(saveButton).hide();

}

function validateBudgetMappings(budgetMappingContainer) {

    let totalAmount = 0;

    $(budgetMappingContainer).find('.budget-mapping__amount').each(function () {
        let input = $(this).find('input');
        totalAmount += Math.round($(input).val() * 100);
    });

    let transaction = $(budgetMappingContainer).parents('.transaction-list__entry');
    let transactionAmount = $(transaction).data('amount');

    if (totalAmount != transactionAmount) {
        alert('Invalid budget mapping amounts: amount budgeted is ' + formatAmount(totalAmount - transactionAmount) + " off from the transaction amount");
        return false;
    }

    return true;
}

function formatAmount(amount) {

    let negative = amount < 0;
    let dollars = Math.floor(Math.abs(amount / 100));
    let cents = Math.abs(amount % 100);

    let output = negative ? "-$" : "$";
    output += dollars.toLocaleString();
    output += "." + (cents < 10 ? "0" + cents : cents);

    return output;
}