export const getTeaFromSlug = (teaData: any, slug: string | string[]) => {
  return teaData.value.find((tea: any) => tea.slug === slug);
};
