import { Component, PropsWithoutRef } from 'react';
import { Accordion, Button, Card, Col, Row, Table } from 'react-bootstrap';
import {
  ChampionshipDriver,
  ChampionshipResults as ChampionshipResultsData,
  ChampionshipType,
  CLASS_MAP,
  ClassChampionshipDriver,
  IndexedChampionshipResults,
  ShortCarClass,
} from '../models';
import {
  calculateClassChampionshipTrophies,
  doesIndexDriverGetATrophy,
  ChampionshipResultsParser,
} from '../services';
import { RamDownload } from './DownloadButton';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDownload } from '@fortawesome/free-solid-svg-icons';
import { EOL } from 'os';

interface ChampionshipResultsProps extends PropsWithoutRef<any> {
  results?: ChampionshipResultsData;
}

interface ChampionshipResultsState {
  downloadName?: string;
  downloadData?: BlobPart;
}

export class ChampionshipResults extends Component<
  ChampionshipResultsProps,
  ChampionshipResultsState
> {
  constructor(props: Readonly<ChampionshipResultsProps>) {
    super(props);
    this.state = {};
  }

  render() {
    if (
      this.props.results &&
      Object.values(this.props.results).some((v) => !!v)
    ) {
      return [
        <Row key={0} className={'top-buffer'}>
          <Col>
            <h2>Championship Standings</h2>

            <Accordion>
              {this.renderClassChampionshipResults()}

              {Object.entries(this.props.results)
                // Class results need to be displayed separately
                .filter(([championshipType, _]) => championshipType !== 'Class')
                // Get some nice type hints going
                .map(
                  ([key, value]) =>
                    [key, value] as [
                      ChampionshipType,
                      IndexedChampionshipResults,
                    ],
                )
                .map(([championshipType, results]) => {
                  return (
                    <Card key={championshipType}>
                      {this.renderCardHeader(championshipType, results.year)}

                      <Accordion.Collapse eventKey={championshipType}>
                        <Card.Body>
                          <Table striped hover borderless>
                            <thead>
                              <tr>
                                <th>Trophy</th>
                                <th>Rank</th>
                                <th>Driver</th>
                                {new Array(results.drivers[0].points.length)
                                  .fill(null)
                                  .map((_, index) => (
                                    <th key={index}>Event #{index + 1}</th>
                                  ))}
                                <th>Total Points</th>
                                <th>
                                  Best{' '}
                                  {ChampionshipResultsParser.calculateEventsToCount(
                                    results.drivers[0].points.length,
                                  )}{' '}
                                  of {results.drivers[0].points.length}
                                </th>
                              </tr>
                            </thead>

                            <tbody>
                              {results.drivers
                                // Reverse sort by doing `d2 - d1`, so top points shows up at the top
                                .sort(
                                  (d1, d2) => d2.totalPoints - d1.totalPoints,
                                )
                                .map((driver, index) => (
                                  <tr key={index}>
                                    <td>
                                      {doesIndexDriverGetATrophy(driver, index)
                                        ? 'T'
                                        : ''}
                                    </td>
                                    <td>{index + 1}</td>
                                    <td>{driver.name}</td>
                                    {driver.points.map((points, index) => (
                                      <td key={index}>{points}</td>
                                    ))}

                                    <td>
                                      {driver.points.reduce(
                                        (sum, p) => sum + p,
                                        0,
                                      )}
                                    </td>

                                    <td>{driver.totalPoints}</td>
                                  </tr>
                                ))}
                            </tbody>
                          </Table>
                        </Card.Body>
                      </Accordion.Collapse>
                    </Card>
                  );
                })}
            </Accordion>
          </Col>
        </Row>,
        <RamDownload
          key={1}
          filename={this.state.downloadName}
          content={this.state.downloadData}
          contentType={'application/vnd.ms-excel'}
          downloadComplete={() =>
            this.setState({ downloadData: undefined, downloadName: undefined })
          }
        />,
      ];
    } else {
      return null;
    }
  }

  renderClassChampionshipResults(): JSX.Element | null {
    const results = this.props.results?.Class;
    if (results) {
      const championshipType: ChampionshipType = 'Class';
      const eventCount = Object.values(results.driversByClass)[0][0].points
        .length;
      return (
        <Card key={championshipType}>
          {this.renderCardHeader(championshipType, results.year)}

          <Accordion.Collapse eventKey={championshipType}>
            <Card.Body>
              <Table striped hover borderless>
                {(Object.keys(results.driversByClass) as ShortCarClass[])
                  .sort()
                  .map((carClass) => {
                    const trophyCount = calculateClassChampionshipTrophies(
                      results?.driversByClass[carClass],
                    );
                    return [
                      <thead key={0}>
                        <tr>
                          <th colSpan={4 + eventCount}>{carClass}</th>
                        </tr>
                        <tr>
                          <th>Trophy</th>
                          <th>Rank</th>
                          <th>Driver</th>
                          {new Array(eventCount).fill(null).map((_, index) => (
                            <th key={index}>Event #{index + 1}</th>
                          ))}
                          <th>Points</th>
                          <th>
                            Best{' '}
                            {ChampionshipResultsParser.calculateEventsToCount(
                              eventCount,
                            )}{' '}
                            of {eventCount}
                          </th>
                        </tr>
                      </thead>,
                      <tbody key={1}>
                        {results!.driversByClass[carClass]
                          // Reverse sort by doing `d2 - d1`, so top points shows up at the top
                          .sort((d1, d2) => d2.totalPoints - d1.totalPoints)
                          .map((driver, index) => (
                            <tr key={index}>
                              <td>{index < trophyCount ? 'T' : ''}</td>
                              <td>{index + 1}</td>
                              <td>{driver.name}</td>
                              {driver.points.map((p, index) => (
                                <td key={index}>{p}</td>
                              ))}
                              <td>
                                {driver.points.reduce((sum, p) => sum + p, 0)}
                              </td>
                              <td>{driver.totalPoints}</td>
                            </tr>
                          ))}
                      </tbody>,
                    ];
                  })}
              </Table>
            </Card.Body>
          </Accordion.Collapse>
        </Card>
      );
    } else {
      return null;
    }
  }

  renderCardHeader(championshipType: ChampionshipType, year: number) {
    return (
      <Card.Header key={championshipType}>
        <Accordion.Toggle
          eventKey={championshipType}
          as={Button}
          variant={'link'}
        >
          {year} {championshipType}
        </Accordion.Toggle>
        <Button
          variant={'secondary'}
          onClick={() =>
            championshipType === 'Class'
              ? this.exportClassesAsCsv()
              : this.exportIndexAsCsv(championshipType)
          }
        >
          <FontAwesomeIcon className={'clickable'} icon={faDownload} />
        </Button>
      </Card.Header>
    );
  }

  private exportClassesAsCsv() {
    const results = this.props.results!.Class!;
    const header = ChampionshipResults.startCsv(
      'Class',
      results.year,
      results.organization,
      Object.values(results.driversByClass)[0][0].points.length,
    );

    const rows = [
      ...header,
      ...(
        Object.entries(results.driversByClass) as [
          ShortCarClass,
          ClassChampionshipDriver[],
        ][]
      )
        .map(([carClass, drivers]) => {
          if (!CLASS_MAP[carClass]) {
            console.error(`Can not map class "${carClass}"`);
          }
          const trophyCount = calculateClassChampionshipTrophies(drivers);
          return [
            [`${carClass} - ${CLASS_MAP[carClass].long}`],
            ...drivers
              .sort((d1, d2) => d2.totalPoints - d1.totalPoints)
              .map((driver, index) =>
                ChampionshipResults.driverToCsv(driver, index, trophyCount),
              ),
          ];
        })
        .flat(),
    ];
    this.setState({
      downloadData: rows.map((row) => `"${row.join('","')}"`).join(EOL),
      downloadName: `${results.year}_StL_Class_Championship.csv`,
    });
  }

  private exportIndexAsCsv(
    championshipType: Exclude<ChampionshipType, 'Class'>,
  ) {
    const results = this.props.results![championshipType]!;
    const header = ChampionshipResults.startCsv(
      championshipType,
      results.year,
      results.organization,
      results.drivers[0].points.length,
    );

    const rows = results.drivers
      .sort((d1, d2) => d2.totalPoints - d1.totalPoints)
      .map((driver, index) => ChampionshipResults.driverToCsv(driver, index));

    this.setState({
      downloadData: [...header, ...rows]
        .map((row) => `"${row.join('","')}"`)
        .join(EOL),
      downloadName: `${results.year}_StL_${championshipType}_Championship.csv`,
    });
  }

  private static startCsv(
    championshipType: ChampionshipType,
    year: number,
    organization: string,
    totalEventCount: number,
  ): string[][] {
    const eventsToCount =
      ChampionshipResultsParser.calculateEventsToCount(totalEventCount);

    return [
      [organization],
      [
        `${year} ${championshipType} Championship -- Best ${eventsToCount} of ${totalEventCount}`,
      ],
      [],
      [
        'Trophy',
        'Rank',
        'Driver',
        ...new Array(totalEventCount)
          .fill(null)
          .map((_, index) => `Event #${index + 1}`),
        'Total Points',
        `Best ${eventsToCount} of ${totalEventCount}`,
      ],
    ];
  }

  private static driverToCsv(
    driver: ChampionshipDriver,
    index: number,
    trophyCount?: number,
  ): string[] {
    let trophy: boolean;
    if (trophyCount === undefined) {
      trophy = doesIndexDriverGetATrophy(driver, index);
    } else {
      trophy = index < trophyCount;
    }
    return [
      trophy ? 'T' : '',
      `${index + 1}`,
      driver.name,
      ...driver.points.map((p) => `${p}`),
      `${driver.points.reduce((sum, p) => sum + p, 0)}`,
      `${driver.totalPoints}`,
    ];
  }
}
