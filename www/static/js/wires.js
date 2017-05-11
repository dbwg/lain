jQuery(document).ready(function ($) {
  $('#nav-toggle').click(function() {
    $(this).toggleClass('is-active');
    $('#nav-menu').toggleClass('is-active');
  });

  $('.modal-button').click(function() {
    var target = $(this).data('target');
    $('html').addClass('is-clipped');
    $(target).addClass('is-active');
  });

  $('.modal-background, .modal-close').click(function() {
    $('html').removeClass('is-clipped');
    $(this).parent().removeClass('is-active');
  });

  $('.modal-card-head .delete, .modal-card-foot .button').click(function() {
    $('html').removeClass('is-clipped');
    $('#modal-ter').removeClass('is-active');
  });

  $(document).on('keyup',function(e) {
    if (e.keyCode == 27) {
      $('html').removeClass('is-clipped');
      $('.modal').removeClass('is-active');
    }
  });
});
