$(document).ready(function() {

    $('.transaction-list__category').each(function() {
        setupCategoryElement($(this));
    });

    setupMenu();
});

function setupMenu() {
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
            $.post('/secure/api/remove-category/' + transactionId, function(transactionDetails) {
                updateCategoryElement(category, transactionDetails);
            });
        });
    }

}

function openCategorySelectMenu(categoryElement) {

    $('#category-select').css('right', '50px');
    $('#category-select').css('top', ($(categoryElement).position().top + $('#content-container').scrollTop() + $(categoryElement).outerHeight()) + 'px');
    $('#category-select').data('transaction-id', $(categoryElement).parents('.transaction-list__entry').data('id'));
    $('#category-select').show();

    $('.menu__item').off('click').on('click', function(e) {
        e.stopPropagation();
        let categoryId = $(this).data('id');
        if (!categoryId) {
            return;
        }

        let transactionId = $('#category-select').data('transaction-id');
        $.post('/secure/api/set-category/' + transactionId + '/' + categoryId, function(transactionDetails) {
            updateCategoryElement(categoryElement, transactionDetails);
        });
    });
}

function updateCategoryElement(categoryElement, transactionDetails) {
    $('#category-select').hide();
    $('#category-select').find('.menu').hide();

    if (transactionDetails.category_id) {
        categoryElement.removeClass('transaction-list__category--uncategorized');
        let textElem = $('<span/>').text(transactionDetails.category_title);
        let removeIcon = $('<span class="transaction-list__category-remove"><i class="fa-solid fa-xmark"></i></span>');
        categoryElement.empty().append(textElem, removeIcon);
    } else {
        categoryElement.addClass('transaction-list__category--uncategorized');
        let textElem = $('<span/>').text('Uncategorized');
        categoryElement.empty().append(textElem);
    }

    setupCategoryElement(categoryElement);
}