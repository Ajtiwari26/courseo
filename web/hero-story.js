(function () {
  'use strict';

  function initHeroStory() {
    const section = document.getElementById('hero-scroll-story');
    if (section) {
      const stages = Array.from(section.querySelectorAll('.cockpit-stage'));
      const steps = Array.from(section.querySelectorAll('.scroll-step'));
      const progressFill = document.getElementById('scroll-progress-fill');
      const totalStages = stages.length;
      let activeIndex = 0;
      let rafId = null;

      const clamp = (val, min, max) => Math.min(Math.max(val, min), max);

      function setStage(index) {
        if (index === activeIndex && stages[index] && stages[index].classList.contains('is-active')) {
          return;
        }
        activeIndex = index;
        section.setAttribute('data-stage', String(index));

        stages.forEach((stage, i) => {
          stage.classList.toggle('is-active', i === index);
          stage.classList.toggle('is-exited', i < index);
          stage.setAttribute('aria-hidden', String(i !== index));
        });

        steps.forEach((step, i) => {
          const isActive = i === index;
          step.classList.toggle('is-active', isActive);
          step.setAttribute('aria-selected', String(isActive));
          step.setAttribute('tabindex', isActive ? '0' : '-1');
        });
      }

      function updateOnScroll() {
        rafId = null;
        const scrollableDistance = section.offsetHeight - window.innerHeight;
        if (scrollableDistance <= 0) {
          setStage(0);
          if (progressFill) progressFill.style.width = '100%';
          return;
        }

        const sectionTop = section.offsetTop;
        const rawProgress = (window.scrollY - sectionTop) / scrollableDistance;
        const progress = clamp(rawProgress, 0, 1);

        if (progressFill) {
          progressFill.style.width = `${(progress * 100).toFixed(2)}%`;
        }

        let targetIndex = 0;
        if (progress >= 0.67) {
          targetIndex = Math.min(2, totalStages - 1);
        } else if (progress >= 0.34) {
          targetIndex = Math.min(1, totalStages - 1);
        } else {
          targetIndex = 0;
        }
        setStage(targetIndex);
      }

      function onScroll() {
        if (rafId === null) {
          rafId = window.requestAnimationFrame(updateOnScroll);
        }
      }

      steps.forEach((step, i) => {
        const scrollToStage = () => {
          const scrollableDistance = section.offsetHeight - window.innerHeight;
          if (scrollableDistance <= 0) {
            setStage(i);
            return;
          }
          const ratios = [0.02, 0.45, 0.85];
          const ratio = ratios[i] !== undefined ? ratios[i] : i / Math.max(1, totalStages);
          const targetTop = section.offsetTop + scrollableDistance * ratio;
          window.scrollTo({ top: targetTop, behavior: 'smooth' });
        };

        step.addEventListener('click', scrollToStage);
        step.addEventListener('keydown', (e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            scrollToStage();
          }
        });
      });

      window.addEventListener('scroll', onScroll, { passive: true });
      window.addEventListener('resize', onScroll, { passive: true });
      setStage(0);
      updateOnScroll();

      window.CourseoHeroStory = {
        setStage,
        getActiveStage: () => activeIndex,
        updateOnScroll
      };
    }

    // Showreel Modal Controls
    const openShowreelBtn = document.getElementById('openShowreelBtn');
    const closeShowreelBtn = document.getElementById('closeShowreelBtn');
    const showreelModal = document.getElementById('showreelModal');
    const showreelBackdrop = document.getElementById('showreelBackdrop');
    const showreelVideo = document.getElementById('showreelVideo');

    function openShowreel() {
      if (!showreelModal) return;
      showreelModal.removeAttribute('hidden');
      if (showreelVideo) {
        showreelVideo.play().catch(() => {});
      }
    }

    function closeShowreel() {
      if (!showreelModal) return;
      showreelModal.setAttribute('hidden', '');
      if (showreelVideo) {
        showreelVideo.pause();
      }
    }

    if (openShowreelBtn) {
      openShowreelBtn.addEventListener('click', openShowreel);
    }
    if (closeShowreelBtn) {
      closeShowreelBtn.addEventListener('click', closeShowreel);
    }
    if (showreelBackdrop) {
      showreelBackdrop.addEventListener('click', closeShowreel);
    }
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && showreelModal && !showreelModal.hasAttribute('hidden')) {
        closeShowreel();
      }
    });

    if (window.CourseoHeroStory) {
      window.CourseoHeroStory.openShowreel = openShowreel;
      window.CourseoHeroStory.closeShowreel = closeShowreel;
    } else {
      window.CourseoHeroStory = { openShowreel, closeShowreel };
    }
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initHeroStory);
  } else {
    initHeroStory();
  }
})();
