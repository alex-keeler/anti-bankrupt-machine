$(document).ready(function() {

    $('.transaction-list__category').each(function() {
        setupCategoryElement($(this));
    });

    setupCategoryMenu();

    $('.transaction-list__user-description').each(function() {
        setupUserDescriptionElement($(this));
    });

    $('.transaction-list__comment').each(function() {
        setupCommentElement($(this));
    });
});

function setupCategoryElement(categoryElement) {
    if ($(categoryElement).hasClass('transaction-list__category--uncategorized')) {
        $(categoryElement).on('mouseenter', function() {
            openCategorySelectMenu($(this));
        });

        $(categoryElement).on('mouseleave', function() {
            if ($('#category-select:hover').length == 0) {
                $('#category-select').hide();
                $('#category-select').find('.menu').hide();
            }
        });
    } else {
        $(categoryElement).off('mouseenter');
        $(categoryElement).off('mouseleave');

        $(categoryElement).find('.transaction-list__category-remove').on('click', function() {
            let category = $(this).parents('.transaction-list__category');
            let transactionId = $(this).parents('.transaction-list__entry').data('id');

            switch (category.data('type')) {
                case "category":
                    $.post('/secure/api/remove-category/' + transactionId, function(transactionDetails) {
                        updateCategoryElement(category, transactionDetails);
                    });
                    break;
                case "budget":
                    $.post('/secure/api/remove-contribution-budget/' + transactionId, function(transactionDetails) {
                        updateCategoryElement(category, transactionDetails);
                    });
                    break;
                default:
                    console.error('unknown category type: ', category.data('type'));
                    break;
            }
        });
    }
}

function setupUserDescriptionElement(userDescriptionElement) {
    let text = $(userDescriptionElement).find('.transaction-list__user-description--text');
    let link = $(userDescriptionElement).find('.transaction-list__user-description--link');
    let input = $(userDescriptionElement).find('.transaction-list__user-description--input');

    $(input).on('keypress', function(e) {
        if (e.keyCode === 13) {
            updateUserDescription(userDescriptionElement);
        }
    });

    $(input).on('focusout', function() {
        updateUserDescription(userDescriptionElement);
    });

    $(text).on('click', function() {
        input.val(text.text());
        showUserDescriptionElement(userDescriptionElement, true);
    });

    $(link).on('click', function() {
        input.val("");
        showUserDescriptionElement(userDescriptionElement, true);
    });

    showUserDescriptionElement(userDescriptionElement, false);
}

function updateUserDescription(userDescriptionElement) {
    let transactionId = $(userDescriptionElement).parents('.transaction-list__entry').data('id');
    let userDescription = $(userDescriptionElement).find('.transaction-list__user-description--input').val();
    $.post('/secure/api/set-user-description/' + transactionId, userDescription, function(transactionDetails) {
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

    $(input).on('keypress', function(e) {
        if (e.keyCode === 13) {
            updateComment(commentElement);
        }
    });

    $(input).on('focusout', function() {
        updateComment(commentElement);
    });

    $(text).on('click', function() {
        input.val(text.text());
        showCommentElement(commentElement, true);
    });

    $(link).on('click', function() {
        input.val("");
        showCommentElement(commentElement, true);
    });

    showCommentElement(commentElement, false);
}

function updateComment(commentElement) {
    let transactionId = $(commentElement).parents('.transaction-list__entry').data('id');
    let comment = $(commentElement).find('.transaction-list__comment--input').val();
    $.post('/secure/api/set-comment/' + transactionId, comment, function(transactionDetails) {
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

function setupCategoryMenu() {
    $('#category-select').on('mouseleave', function() {
        $('#category-select').hide();
        $('#category-select').find('.menu').hide();
    });

    $('.menu__item').on('mouseenter', function() {
        $('#category-select').find('.menu').not($(this).parent()).hide();
        let submenu = $(this).find('.menu');
        if (submenu.length != 0) {
            submenu.css('top', $(this).position().top + "px");
            submenu.css('right', $(this).outerWidth() + "px");
            submenu.show();
        }
    });

    $('#category-select').on('mouseleave', function() {
        $('#category-select').hide();
        $('#category-select').find('.menu').hide();
    });
}

function openCategorySelectMenu(categoryElement) {

    $('#category-select').css('right', '50px');
    $('#category-select').css('top', ($(categoryElement).position().top + $('#content-container').scrollTop() + $(categoryElement).outerHeight()) + 'px');
    $('#category-select').data('transaction-id', $(categoryElement).parents('.transaction-list__entry').data('id'));
    $('#category-select').show();

    $('.menu__item').off('click').on('click', function(e) {
        e.stopPropagation();

        let transactionId = $('#category-select').data('transaction-id');
        let categoryId = $(this).data('id');

        switch ($(this).data('type')) {
            case "category":
                $.post('/secure/api/set-category/' + transactionId + '/' + categoryId, function(transactionDetails) {
                    updateCategoryElement(categoryElement, transactionDetails);
                });
                break;
            case "budget":
                $.post('/secure/api/set-contribution-budget/' + transactionId + '/' + categoryId, function(transactionDetails) {
                    updateCategoryElement(categoryElement, transactionDetails);
                });
                break;
            default:
                console.error('Unknown category type: ', $(this).data('type'));
                break;
        }
    });
}

function updateCategoryElement(categoryElement, transactionDetails) {
    $('#category-select').hide();
    $('#category-select').find('.menu').hide();

    if (transactionDetails.category_id) {
        categoryElement.removeClass('transaction-list__category--uncategorized');
        categoryElement.data('type', 'category');
        let textElem = $('<span/>').text(transactionDetails.category_title);
        let removeIcon = $('<span class="transaction-list__category-remove"><i class="fa-solid fa-xmark"></i></span>');
        categoryElement.empty().append(textElem, removeIcon);
    } else if (transactionDetails.contribution_budget_id) {
        categoryElement.removeClass('transaction-list__category--uncategorized');
        categoryElement.data('type', 'budget');
        console.log(transactionDetails.contribution_budget_classification);
        let text = transactionDetails.contribution_budget_classification === 'Saving' ?
            'Saving - ' + transactionDetails.contribution_budget_title :
            'Financing - ' + transactionDetails.contribution_budget_title;
        let textElem = $('<span/>').text(text);
        let removeIcon = $('<span class="transaction-list__category-remove"><i class="fa-solid fa-xmark"></i></span>');
        categoryElement.empty().append(textElem, removeIcon);
    } else {
        categoryElement.addClass('transaction-list__category--uncategorized');
        categoryElement.data('type', '');
        let textElem = $('<span/>').text('Uncategorized');
        categoryElement.empty().append(textElem);
    }

    setupCategoryElement(categoryElement);
}