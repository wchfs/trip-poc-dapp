<template>
    <section
      id="faq"
      aria-labelledby="faq-title"
      class="relative overflow-hidden bg-slate-50 py-20 sm:py-32"
    >
      <img
        class="absolute top-0 left-1/2 max-w-none translate-x-[-30%] -translate-y-1/4"
        src="/images/background-faqs.jpg"
        alt=""
        width=1558
        height=946
        unoptimized
      />
      <Container className="relative">
        <div class="mx-auto max-w-2xl lg:mx-0">
          <h2
            id="faq-title"
            class="font-display text-3xl tracking-tight text-slate-900 sm:text-4xl"
          >
            Frequently asked questions
          </h2>
          <p className="mt-4 text-lg tracking-tight text-slate-700">
            If you can’t find what you’re looking for, email our support team
            and if you’re lucky someone will get back to you.
          </p>
        </div>
        <ul
          role="list"
          class="mx-auto mt-16 grid max-w-2xl grid-cols-1 gap-8 lg:max-w-none lg:grid-cols-3"
        >
            <li v-for="( faq ) in faqs">
              <ul role="list" class="flex flex-col gap-y-8">
                  <li v-for="( question ) in faq">
                    <h3 class="font-display text-lg leading-7 text-slate-900">
                      {{ question.question}}
                    </h3>
                    <p class="mt-4 text-sm text-slate-700">{{ question.answer }}</p>
                  </li>
              </ul>
            </li>
        </ul>
      </Container>
    </section>
</template>

<script setup lang="ts">
import Container from "@/components/Landing/Container.vue";

const faqs = [
  [
    {
      question: 'Does Webchefs team is awesome?',
      answer:
        'Of course! We specialized in Web development and still learning new things!',
    },
  ],
  [
    {
      question: 'Does Blockchain OS is useful?',
      answer:
        'Blockchain OS is providing a great set of tools that makes Web3 development much easier.',
    },
  ],
  [
    {
      question: 'How can I contact you?',
      answer:
        'Please refer to our page, you can find it on this page!',
    },
  ],
]
</script>
